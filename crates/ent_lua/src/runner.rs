// ./crates/ent_lua/src/runner.rs
use crate::ScriptEngine;
use crate::script_engine::AutomationEvent;
use ent_core::handle::EntityHandle;
use ent_types::SystemMessage; // Работаем с единым типом сообщений
use tokio::sync::mpsc::UnboundedReceiver;
use std::path::Path;

pub struct ScriptRunner;

impl ScriptRunner {
    /// Спавнит ScriptRunner в отдельном потоке ОС.
    /// Он принимает rx_for_lua (рельсы от Роутера) и EntityHandle (для get_state из Lua).
    pub fn spawn(
        scripts_dir: impl AsRef<Path> + Send + 'static,
        mut rx_for_lua: UnboundedReceiver<SystemMessage>,
        tx_handle: EntityHandle,
    ) {
        std::thread::Builder::new()
            .name("[entcore] script-runner".into())
            .spawn(move || {
                // Создаем локальный рантайм для чтения из асинхронного канала rx_for_lua
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .expect("Не удалось запустить рантайм для ScriptRunner");

                rt.block_on(async {
                    let (mut engine, receiver) = ScriptEngine::new(tx_handle)
                        .expect("Ошибка инициализации ScriptEngine");
                    
                    engine.register_log().unwrap();
                    engine.registr_entity_id().unwrap();
                    engine.get_state().unwrap();
                    
                    engine.load_scripts(scripts_dir, receiver)
                        .expect("Не удалось загрузить Lua скрипты");

                    println!("[Lua] Движок автоматизации запущен и готов к событиям.");

                    // Читаем сообщения, прилетающие от центрального Роутера
                    while let Some(msg) = rx_for_lua.recv().await {
                        match msg {
                            // Роутер будит нас, если какая-то сущность изменилась!
                            SystemMessage::EntityChangedEvent { entity_id } => {
                                // Передаем событие в Lua. Пока передаем 0.0, 
                                // так как Lua сам пойдет в get_state за нужными полями
                                let event = AutomationEvent {
                                    entity_id,
                                    value: 0.0, 
                                };
                                let _ = engine.call_functions(event);
                            }
                            SystemMessage::Shutdown => {
                                println!("[Lua] Получен сигнал Shutdown. Остановка скриптов...");
                                break;
                            }
                            _ => {} // Все остальные сообщения Lua-движок игнорирует
                        }
                    }
                });
            })
            .expect("Не удалось запустить поток ScriptRunner");
    }
}
