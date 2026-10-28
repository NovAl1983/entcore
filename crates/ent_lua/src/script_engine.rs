
use mlua::{Lua, Function, UserData};
use std::{collections::HashMap, path::Path, println, sync::mpsc::{self, Receiver}, time::Duration};
use crate::source;
use ent_core::{EntityState, LightState, handle::EntityHandle};
use ent_core::state::SensorState;
use ent_core::EntityManager;

#[derive(UserData, Clone)]
pub struct AutomationEvent {
    pub entity_id: String,
    pub value: f64,
}



pub struct  ScriptEngine {
    pub lua: Lua,
    pub functions: Vec<Function>,
    pub subscriptions: HashMap<String, Vec<usize>>, // entity_id -> Vec<usize>> где usize это номер элемента вектора в  pub functions: Vec<Function>,
    pub tx: mpsc::Sender<Vec<String>>,
    pub tx_handle: EntityHandle,

}

impl ScriptEngine {
    pub fn new(tx_handle: EntityHandle) -> Result<(Self, mpsc::Receiver<Vec<String>>), mlua::Error> {
        let lua = Lua::new();
        let (tx, rx) = mpsc::channel();
        Ok((Self {
            lua, 
            functions: Vec::new(),
            subscriptions:HashMap::new(),
            tx,
            tx_handle 
        }, rx))
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


        pub fn registr_entity_id(&self)-> Result<(), mlua::Error> {
            let tx = self.tx.clone();
            let f = self.lua.create_function(move|_, ids: Vec<String>| {
            tx.send(ids).unwrap();
            Ok(())
        })?;
        self.lua.globals().set("registr_entity_id", f)?;
        Ok(())
        }

        pub fn get_state(&self) -> Result<(), mlua::Error> {
            let handle = self.tx_handle.clone();
            let lua = self.lua.clone(); 
            let f = self.lua.create_function(move |_, entity_id: String| {
                match handle.get_state_blocking(&entity_id) {
                    Some(e) => match e.state {
                        EntityState::Light(l) => Ok(mlua::Value::UserData(lua.create_userdata(l)?)),
                        EntityState::Sensor(s) => Ok(mlua::Value::Number(s.value)),
                        EntityState::BinarySensor(b) => Ok(mlua::Value::Boolean(b.is_on)),
                        EntityState::InputBoolean(i) => Ok(mlua::Value::Boolean(i.is_on)),
                        EntityState::Counter(c) => Ok(mlua::Value::Number(c.value as f64)),
        },
                    None => Ok(mlua::Value::Nil),
    }
})?;
        self.lua.globals().set("get_state", f)?; 
        Ok(())
        }

}


