
use std::{collections::HashMap, path::{Path, PathBuf}, println, sync::mpsc::{Receiver, Sender}, time::Duration};
use cfg::Config;
use ent_lua::source;
use std::sync::mpsc;

use mlua::{Lua, Function, UserData};


pub fn registr_entity_id(ids: Vec<String>, tx: Sender<Vec<String>>) {
    tx.send(ids).unwrap();
}



#[derive(UserData, Clone)]
struct AutomationEvent {
    entity_id: String,
    value: f64,
}



struct  ScriptEngine {
    lua: Lua,
    functions: Vec<Function>,
    subscriptions: HashMap<String, Vec<usize>>, // name_script -> vec<entity_id>
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

    pub fn load_scripts(&mut self, dir: impl AsRef<Path>, rx: Receiver<Vec<String>>) -> Result<(), Box<dyn std::error::Error>> {
        let scripts: Vec<source::Script> = source::load_scripts(dir)?;
        for script in scripts {
            let func: Function = self.lua.load(&script.source_code)
                .set_name(&script.name)   // ← полезно для ошибок
                .eval()?;
            let num = self.functions.len();
            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok(ids) => {
                    for id in ids {
                        self.subscriptions.entry(id).or_default().push(num);
                    } 
                },
                Err(e) => {
                    println!("Ошибка ({}) вызова функции registr_entity_id в скрипте {}", e, script.name);
                    continue;
                }
            }
            self.functions.push(func);

        }
        Ok(())
    }

    pub fn call_all_functions(&self, event: AutomationEvent) -> Result<(), mlua::Error> {
        for f in &self.functions {
            f.call::<()>(event.clone())?;
        }
        Ok(())
    }
    pub fn call_functions(&self, event: AutomationEvent) -> Result<(), mlua::Error> {
        let functions = self.subscriptions.get(&event.entity_id);
        match functions {
            Some(functions) => {
                for &f in functions {
                    self.functions[f].call::<()>(event.clone())?;
                }
            },
            None => {println!("Скрипты не подписаны на сущность {}", &event.entity_id) }
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // канал для функции registr_entity_id_fn
    let (tx, rx) = mpsc::channel::<Vec<String>>();

    // 1. Создаём Lua state
    let mut engine = ScriptEngine::new()?;

    // 2. Регистрируем функцию log в глобальном пространстве Lua
    engine.register_log()?;

    let registr_entity_id_fn = engine.lua.create_function(move|_, ids: Vec<String>| {
            let tx_clone: Sender<Vec<String>> = tx.clone();
            registr_entity_id(ids, tx_clone);
            Ok(())
        })?;
        engine.lua.globals().set("registr_entity_id", registr_entity_id_fn)?;


    let dir: PathBuf = { 
        let cfg: Config = Config::load_config("config.toml")
        .map_err(|e| format!("Ошибка загрузки конфига: {}", e))?;
        PathBuf::from(&cfg.system.scripts_directory)
    };

    engine.load_scripts(dir, rx)?;

    let event = AutomationEvent {
        entity_id: "light.kitchen".to_string(),
        value: 42.0,
    };
    engine.call_functions(event)?;

    Ok(())
}