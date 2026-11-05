use ent_types::EntityState;
use mlua::UserData;
use std::collections::HashMap;



#[derive(Debug, Clone, PartialEq, UserData)]
pub enum StateKind {
    Light,
    Sensor,
    BinarySensor,
    Counter,
    InputBoolean,
}

impl StateKind {
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
    pub fn new() -> Self {
        Self {map: HashMap::new() }
    }
}
