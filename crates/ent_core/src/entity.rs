
// use crate::state::{EntityState, LightState, SensorState, BinarySensorState, CounterState, InputBooleanState};
use crate::state::{EntityState,};

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: String,
    pub friendly_name: String,
    pub state: EntityState,
    // pub state_filed_kind: LightStateFiledKind,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_entity() {
// Было: state: EntityState::Light { is_on: false, brightness: None, color_temp: None }
// Стало:
use crate::state::{EntityState, LightState,};

      let e = Entity {
            id: "light.living_room".to_string(),
            friendly_name: "Люстра в зале".to_string(),
            state: EntityState::Light(LightState {
                is_on: false,
                brightness: None,
                color_temp: None,
    }),
            // state_filed_kind: LightStateFiledKind::new(),
};

        assert_eq!(e.id, "light.living_room");
        // assert_eq!(e.state, EntityState::Light {
        //     is_on: false,
        //     brightness: Some(128),
        //     color_temp: None,
        // });
        assert_eq!(e.state, EntityState::Light(LightState {
            is_on: false,
            brightness: Some(128),
            color_temp: None,
        })
    );
    }
}