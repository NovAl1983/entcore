use crate::ScriptEngine;
use crate::script_engine::{AutomationEvent};
use std::sync::mpsc;
use std::path::Path;
use ent_core::handle::EntityHandle;


pub enum ScriptRunnerMessage {
    EntityChanged(AutomationEvent),
    Shutdown,
}

pub struct ScriptRunnerHandle {
    tx: mpsc::Sender<ScriptRunnerMessage>,
}

impl ScriptRunnerHandle {
    pub fn notify_changed(&self, event: AutomationEvent) {
        let _ = self.tx.send(ScriptRunnerMessage::EntityChanged(event));
    }
    pub fn shutdown(&self) {
        let _ = self.tx.send(ScriptRunnerMessage::Shutdown);
    }
}

pub struct ScriptRunner;

impl ScriptRunner {
    pub fn spawn(scripts_dir: impl AsRef<Path> + Send + 'static, tx_handle: EntityHandle) -> ScriptRunnerHandle {
        let (tx, rx) = mpsc::channel();
        
        std::thread::Builder::new()
            .name("script-runner".into())
            .spawn(move || {
                // let mut engine = ScriptEngine::new().expect(r"ошибка в crates\ent_lua\src\runner.rs - fn spawn");
                let (mut engine, receiver) = ScriptEngine::new(tx_handle).expect(r"ошибка в crates\ent_lua\src\runner.rs - fn spawn");
                engine.register_log().expect(r"ошибка в crates\ent_lua\src\runner.rs - fn spawn");
                engine.registr_entity_id().expect(r"ошибка в crates\ent_lua\src\runner.rs - fn spawn");
                engine.get_state().expect(r"ошибка в crates\ent_lua\src\runner.rs - fn spawn");
                engine.load_scripts(scripts_dir, receiver).expect(r"ошибка в crates\ent_lua\src\runner.rs - fn spawn");

            loop {
                match rx.recv() {
                    Ok(ScriptRunnerMessage::EntityChanged(event)) => {
                        let _ = engine.call_functions(event);
                    }
                    Ok(ScriptRunnerMessage::Shutdown) => break,
                    Err(e) => {
                        println!("Канал закрыт: {}", e);
                        break;
        }
    }
}
            })
            .expect(r"ошибка в crates\ent_lua\src\runner.rs - fn spawn");
        ScriptRunnerHandle { tx }
    }
}