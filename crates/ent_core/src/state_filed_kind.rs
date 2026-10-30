use std::collections::HashMap;
use mlua::UserData;

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

pub struct StateFiledKind {
    pub map: HashMap<String, StateKind>,
}

impl StateFiledKind {
    pub fn new(&mut self, entities: &EntityManager) -> Self {
        let mut map:HashMap<String, StateKind> = HashMap::new();
        for (ent_id, entity) in entities.entities.clone() {
            match entity.state {
                Light(_) => {map.insert(ent_id.to_string(), StateKind::Light);}
                Sensor(_) =>{map.insert(ent_id.to_string(), StateKind::Sensor);}
                BinarySensor(_) => {map.insert(ent_id.to_string(), StateKind::BinarySensor);}
                Counter(_)=> {map.insert(ent_id.to_string(), StateKind::Counter);}
                InputBoolean(_) => {map.insert(ent_id.to_string(), StateKind::InputBoolean);}
            }
        }
       Self{map} 
    }
}

