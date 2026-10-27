
use ent_lua::AutomationEvent;
use std::time::Duration;
use ent_lua::runner::ScriptRunner;
use ent_core::{EntityManager, EntityState, LightState,Entity};



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "D://Rust//projects//entcore//scripts";  // из конфига
    let handle = EntityManager::spawn();
    handle.register(Entity {
    id: "light.kitchen".to_string(),
    friendly_name: "Кухня".to_string(),
    state: EntityState::Light(LightState { is_on: true, brightness: Some(200), color_temp: None }),
});
std::thread::sleep(Duration::from_millis(50)); // ← дать менеджеру обработать
    let runner = ScriptRunner::spawn(dir, handle);

    // подождать, пока загрузится (пока — sleep)
    std::thread::sleep(Duration::from_millis(500));

    runner.notify_changed(AutomationEvent {
        entity_id: "light.kitchen".into(),
        value: 42.0,
    });

    std::thread::sleep(Duration::from_millis(500));
    runner.shutdown();

    Ok(())
}