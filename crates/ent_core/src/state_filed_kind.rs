use std::collections::HashMap;
use mlua::UserData;
use crate::Entity;
use crate::EntityState;

use crate::{EntityManager, EntityState::{BinarySensor, Counter, InputBoolean, Light, Sensor}};
// use crate::state::FiledKind;

#[derive(Debug, Clone, PartialEq, UserData)]
pub enum StateKind {
    Light,
    Sensor,
    BinarySensor,
    Counter,
    InputBoolean,
} 

impl  StateKind {
    pub fn from_state(state: &EntityState) -> Self {
        match state {
            EntityState::Light(_) => StateKind::Light,
            EntityState::Sensor(_) => StateKind::Sensor,
            EntityState::BinarySensor(_) => StateKind::BinarySensor,
            EntityState::Counter(_) => StateKind::Counter,
            EntityState::InputBoolean(_) => StateKind::InputBoolean,
        }
    }

    
}

pub struct StateFiledKind {
    pub map: HashMap<String, StateKind>,
}

impl StateFiledKind {
    // pub fn new( entities: HashMap<String, Entity>) -> Self {
    //     let mut map:HashMap<String, StateKind> = HashMap::new();
    //     for (ent_id, entity) in entities {
    //         match entity.state {
    //             Light(_) => {map.insert(ent_id.to_string(), StateKind::Light);}
    //             Sensor(_) =>{map.insert(ent_id.to_string(), StateKind::Sensor);}
    //             BinarySensor(_) => {map.insert(ent_id.to_string(), StateKind::BinarySensor);}
    //             Counter(_)=> {map.insert(ent_id.to_string(), StateKind::Counter);}
    //             InputBoolean(_) => {map.insert(ent_id.to_string(), StateKind::InputBoolean);}
    //         }
    //     }
    //    Self{map} 
    // }
    pub fn new() -> Self {
        let map:HashMap<String, StateKind> = HashMap::new();

       Self{map} 
    }

    
}


// // вызываем функцию в crates\ent_core\src\manager.rs 
// pub fn push(entity: &Entity) {
//     let ent_id = entity.id.clone();
//         match entity.state {
//             Light(_) => {
//                let msg = (ent_id, StateKind::Light);
//                // отправляем через канал в crates\ent_lua\src\runner.rs
//                //
//             }
//             Sensor(_) =>{
//                 let msg: (String, StateKind) = (ent_id, StateKind::Sensor);
//             }           
//             BinarySensor(_) => {
//                 let msg: (String, StateKind) = (ent_id, StateKind::BinarySensor);
//             }
//             Counter(_)=> {
//                 let msg: (String, StateKind) = (ent_id, StateKind::Counter);
//             }
//             InputBoolean(_) => {
//                 let msg: (String, StateKind) = (ent_id, StateKind::InputBoolean);
//             }
//         }
// }