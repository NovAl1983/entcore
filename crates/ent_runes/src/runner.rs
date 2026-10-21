use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc;

use rune::Vm;
use ent_core::{Entity, EntityHandle};
use crate::source::load_scripts;
use crate::compiler::{setup_context, compile_scripts};
use crate::rule::{Rule, AutomationEvent, TriggerKind};

/// Сообщения для управления потоком ScriptRunner
#[derive(Debug)]
pub enum ScriptRunnerMessage {
    /// Уведомление из ядра, что состояние сущности изменилось
    EntityChanged(Entity),
    /// Сигнал к плавному завершению работы
    Shutdown,
}

/// Пульт управления для ScriptRunner
#[derive(Clone)]
pub struct ScriptRunnerHandle {
    tx: mpsc::UnboundedSender<ScriptRunnerMessage>,
}

impl ScriptRunnerHandle {
    pub fn notify_changed(&self, entity: Entity) {
        let _ = self.tx.send(ScriptRunnerMessage::EntityChanged(entity));
    }

    pub fn shutdown(&self) {
        let _ = self.tx.send(ScriptRunnerMessage::Shutdown);
    }
}

pub struct ScriptRunner {
    rx: mpsc::UnboundedReceiver<ScriptRunnerMessage>,
    entity_handle: EntityHandle,
    cache: HashMap<String, Entity>,
    rules: Vec<Rule>,
    rune_runtime: Arc<rune::runtime::RuntimeContext>,
}

impl ScriptRunner {
    /// Запускает ScriptRunner в отдельном OS-потоке (std::thread)
    pub fn spawn(entity_handle: EntityHandle, scripts_dir: String) -> ScriptRunnerHandle {
        let (tx, rx) = mpsc::unbounded_channel();
        let handle = ScriptRunnerHandle { tx };

        thread::Builder::new()
            .name("script-runner".into())
            .spawn(move || {
                let context = setup_context().expect("Не удалось инициализировать контекст Rune");
                let rune_runtime = Arc::new(context.runtime().expect("Не удалось создать рантайм Rune"));

                // Чистый runner без mut, так как владение сразу уходит в метод init_and_run
                let runner = Self {
                    rx,
                    entity_handle,
                    cache: HashMap::new(),
                    rules: Vec::new(),
                    rune_runtime,
                };

                runner.init_and_run(scripts_dir, context);
            })
            .expect("Не удалось запустить поток ScriptRunner");

        handle
    }

    fn init_and_run(mut self, scripts_dir: String, context: rune::Context) {
        println!("🚀 [ScriptRunner] Запуск движка автоматизаций...");

        // 1. Первичное наполнение кэша
        println!("📦 [ScriptRunner] Запрос снимка всех сущностей для кэша...");
        // (Задел под list_all)

        // 2. Загружаем и компилируем скрипты автоматизаций
        let scripts = load_scripts(scripts_dir).unwrap_or_else(|e| {
            eprintln!("⚠️ [ScriptRunner] Ошибка чтения папки скриптов: {}", e);
            Vec::new()
        });

        if !scripts.is_empty() {
            println!("📜 [ScriptRunner] Компиляция {} скриптов...", scripts.len());
            match compile_scripts(&context, &scripts) {
                Ok(unit) => {
                    let mut vm = Vm::new(self.rune_runtime.clone(), unit);
                    
                    for script in &scripts {
                        let item_path = [script.name.as_str(), "init"];
                        if let Ok(value) = vm.call(item_path, ()) {
                            if let Ok(rule) = rune::from_value::<Rule>(value) {
                                println!("✅ [ScriptRunner] Успешно загружено правило автоматизации из {}", script.name);
                                self.rules.push(rule);
                            }
                        }
                    }
                }
                Err(e) => eprintln!("❌ [ScriptRunner] Ошибка компиляции скриптов: {}", e),
            }
        }

        println!("🟢 [ScriptRunner] Движок готов к обработке событий. Правил в базе: {}", self.rules.len());

        // 3. Бесконечный цикл обработки входящих событий умного дома
        while let Some(msg) = self.rx.blocking_recv() {
            match msg {
                ScriptRunnerMessage::EntityChanged(entity) => {
                    let entity_id = entity.id.clone();
                    
                    // Обновляем локальный кэш
                    self.cache.insert(entity_id.clone(), entity.clone());
                    
                    // Вытаскиваем числовое значение для контекста AutomationEvent (задел)
                    let current_value = 1.0; 

                    // Проверяем наши правила на совпадение триггеров
                    for rule in &self.rules {
                        for trigger in &rule.when {
                            if trigger.kind == TriggerKind::Entity && trigger.id == entity_id {
                                println!("⚡ [ScriptRunner] Триггер совпал для '{}'! Запуск автоматизации...", entity_id);
                                
                                // Создаем контекст события для передачи в замыкание `then`
                                let event = AutomationEvent {
                                    trigger: trigger.clone(),
                                    entity_id: Some(entity_id.clone()),
                                    value: current_value,
                                };

                                // Вызываем через .call::<()>((event,)), превращая VmResult в обычный Result с помощью .into_result()
                                if let Err(e) = rule.then.call::<()>((event,)).into_result() {
                                    eprintln!("❌ [ScriptRunner] Ошибка выполнения автоматизации: {}", e);
                                }

                            }
                        }
                    }
                }

                ScriptRunnerMessage::Shutdown => {
                    println!("🏁 [ScriptRunner] Получен сигнал Shutdown. Останов потока скриптов.");
                    break;
                }
            }
        }
    }
}
