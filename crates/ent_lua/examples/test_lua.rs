
use ent_lua::AutomationEvent;
use std::time::Duration;
use ent_lua::runner::ScriptRunner;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = "D://Rust//projects//entcore//scripts";  // из конфига
    let runner = ScriptRunner::spawn(dir);

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