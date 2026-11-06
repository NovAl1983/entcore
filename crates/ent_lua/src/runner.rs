// ./crates/ent_lua/src/runner.rs
use crate::ScriptEngine;
use crate::script_engine::AutomationEvent;
use ent_core::handle::EntityHandle;
use ent_types::SystemMessage;
use std::path::Path;
use tokio::sync::mpsc::UnboundedReceiver;

pub struct ScriptRunner;

impl ScriptRunner {
    /// Спавнит ScriptRunner в отдельном потоке ОС БЕЗ рантайма Tokio
    pub fn spawn(
        scripts_dir: impl AsRef<Path> + Send + 'static,
        mut rx_for_lua: UnboundedReceiver<SystemMessage>,
        tx_handle: EntityHandle,
    ) {
        std::thread::Builder::new()
            .name("[entcore] script-runner".into())
            .spawn(move || {
                // Инициализируем синхронный движок Lua
                let (mut engine, receiver) = match ScriptEngine::new(tx_handle) {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("[Lua] Ошибка создания ScriptEngine: {}", e);
                        return;
                    }
                };

                if let Err(e) = engine.register_log() {
                    eprintln!("[Lua] Ошибка register_log: {}", e);
                    return;
                }
                if let Err(e) = engine.registr_entity_id() {
                    eprintln!("[Lua] Ошибка registr_entity_id: {}", e);
                    return;
                }
                if let Err(e) = engine.get_state() {
                    eprintln!("[Lua] Ошибка get_state: {}", e);
                    return;
                }
                if let Err(e) = engine.set_state() {
                    eprintln!("[Lua] Ошибка set_state: {}", e);
                    return;
                }
                if let Err(e) = engine.load_scripts(scripts_dir, receiver) {
                    eprintln!("[Lua] Ошибка load_scripts: {}", e);
                    // ← можно продолжить, часть скриптов загрузилась
                }

                println!("[Lua] Движок автоматизации запущен и готов к событиям.");

                // Чистый синхронный цикл! Поток спит, пока Роутер не пришлет событие.
                // reply_rx.blocking_recv() внутри get_state теперь НЕ будет вызывать панику,
                // так как мы больше не находимся под управлением рантайма Tokio!
                while let Some(msg) = rx_for_lua.blocking_recv() {
                    match msg {
                        SystemMessage::EntityChangedEvent { entity_id } => {
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
                        _ => {}
                    }
                }
            })
            .expect("Не удалось запустить поток ScriptRunner");
    }
}
