
use std::{collections::HashMap, path::{Path, PathBuf}, println};
use cfg::Config;
use ent_lua::source;

use mlua::{Lua, Function, UserData};




#[derive(UserData, Clone)]
struct AutomationEvent {
    entity_id: String,
    value: f64,
}

#[derive(Debug)]
struct FunctionsMap {
    script_name: String,
    function: Function
} 

struct  ScriptEngine {
    lua: Lua,
    functions: Vec<FunctionsMap>,
    subscriptions: HashMap<String, Vec<String>>,  // name_script -> vec<entity_id>
}

impl ScriptEngine {
    pub fn new() -> Result<Self, mlua::Error> {
        let lua = Lua::new();
        Ok(Self {
            lua, 
            functions: Vec::new(),
            subscriptions:HashMap::new(),
        })
    }

    pub fn load_scripts(&mut self, dir: impl AsRef<Path>) -> Result<(), Box<dyn std::error::Error>> {
        let scripts: Vec<source::Script> = source::load_scripts(dir)?;
        for script in scripts {
            let func: Function = self.lua.load(&script.source_code)
                .set_name(&script.name)   // ← полезно для ошибок
                .eval()?;
            let name = script.name;

            self.functions.push(FunctionsMap { script_name: name, function: func });
            // self.subscriptions.insert(script.name, v)
        }
        Ok(())
    }

    pub fn call_functions(&self, event: AutomationEvent) -> Result<(), mlua::Error> {
        for f in &self.functions {
            f.function.call::<()>(event.clone())?;
        }
        Ok(())
    }

        pub fn register_log(&self) -> Result<(), mlua::Error> {
        let log_fn = self.lua.create_function(|_, msg: String| {
            println!("[Lua] {}", msg);
            Ok(())
        })?;
        self.lua.globals().set("log", log_fn)?;
        Ok(())
    }
}

// #[derive(UserData, Clone)]
// struct Subscriptions {
//     sub: HashMap<String, Vec<String>> // name_script -> entity_id
// }

// pub fn register_entity_id(entity_ids: Vec<String>) -> Vec<String> {
// entity_ids
// }



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Создаём Lua state
    let mut engine = ScriptEngine::new()?;

    // 2. Регистрируем функцию log в глобальном пространстве Lua
    engine.register_log()?;

    let dir: PathBuf = { 
        let cfg: Config = Config::load_config("config.toml")
        .map_err(|e| format!("Ошибка загрузки конфига: {}", e))?;
        PathBuf::from(&cfg.system.scripts_directory)
    };

    engine.load_scripts(dir)?;


    let event = AutomationEvent {
        entity_id: "light.kitchen".to_string(),
        value: 42.0,
    };
    engine.call_functions(event)?;

    println!("{:?}", engine.functions);

    Ok(())


}