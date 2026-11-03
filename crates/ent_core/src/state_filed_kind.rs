use crate::Entity;
use crate::EntityState;
use mlua::UserData;
use std::collections::HashMap;

use crate::{
    EntityManager,
    EntityState::{BinarySensor, Counter, InputBoolean, Light, Sensor},
};
// use crate::state::FiledKind;

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
        let map: HashMap<String, StateKind> = HashMap::new();

        Self { map }
    }
}
