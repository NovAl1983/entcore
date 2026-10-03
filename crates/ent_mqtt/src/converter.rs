
use cfg::Mqtt;
use ent_core::{Entity, EntityState};

//-> Vec<Entity>
pub fn mqtt_entities_from_config(mqtt: &Mqtt) -> Vec<Entity> {
    let mut entities: Vec<Entity> = Vec::new();

    for item in &mqtt.light {
        let entity = Entity {
            id: item.entity_id.clone(),
            friendly_name: item.name.clone(),
            state: EntityState::Light { is_on: false, brightness: None, color_temp: None }
        };
        entities.push(entity); 
    }

    for item in &mqtt.sensor {
        let entity = Entity {
            id: item.entity_id.clone(),
            friendly_name: item.name.clone(),
            state: EntityState::Sensor { value: 0.0 }
        };
        entities.push(entity); 
    }

    entities

}



// mqtt: Mqtt {
//         light: [
//             MqttLight {
//                 entity_id: "light.living_room",
//                 name: "люстра в зале",
//                 state_topic: "/devices/wb-mr6cv3_189/controls/K2",
//                 command_topic: "/devices/wb-mr6cv3_189/controls/K2/on",
//                 brightness_state_topic: "/devices/wb-mr6cv3_189/controls/Brightness",
//                 brightness_command_topic: "/devices/wb-mr6cv3_189/controls/Brightness/on",
//                 brightness_scale: 254,
//             },
//         ],
//         sensor: [
//             MqttSensor {
//                 entity_id: "sensor.kitchen_motion",
//                 name: "датчик движения кухня",
//                 state_topic: "/devices/wb-msw-v4_220/controls/Current Motion",
//             },
//             MqttSensor {
//                 entity_id: "sensor.kitchen_illuminance",
//                 name: "датчик освещенности кухня",
//                 state_topic: "/devices/wb-msw-v4_200/controls/Illuminance",
//             },
//         ],
//     },