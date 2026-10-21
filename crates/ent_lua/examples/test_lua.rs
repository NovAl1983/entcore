use std::println;

use mlua::{Lua, Function, Table};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Создаём Lua state
    let lua = Lua::new();

    // 2. Регистрируем функцию log в глобальном пространстве Lua
    let log_fn = lua.create_function(|_, msg: String| {
        println!("[Lua] {}", msg);
        Ok(())
    })?;
    lua.globals().set("log", log_fn)?;

    // 3. Загружаем скрипт, который возвращает функцию
    let script = r#"
        return function(event)
            log("получено событие: " .. event.entity_id)
            log("значение: " .. tostring(event.value))
        end
    "#;

    let func: Function = lua.load(script).eval()?;

    // 4. Создаём событие (таблицу Lua) и вызываем функцию
    let event = lua.create_table()?;
    event.set("entity_id", "light.kitchen")?;
    event.set("value", 42.0)?;

    func.call::<()>(event)?;

    Ok(())


}