
use cfg::Mqtt;
use ent_core::{Entity, EntityState};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TopicKind {
    LightIsOn,
    LightBrightness,
    LightColorTemp,
    SensorValue,
}

#[derive(Debug, Clone)]
pub struct FieldMapping {
    pub entity_id: String,
    pub kind: TopicKind,
}
pub struct Topics {
    pub state_topic: HashMap<String, FieldMapping>,   // topic → mapping
}


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

pub fn mqtt_from_config(mqtt: &Mqtt) -> Topics {
    let mut topics = Topics { state_topic: HashMap::new() };

    for light in &mqtt.light {
        topics.state_topic.insert(
            light.state_topic.clone(),
            FieldMapping {
                entity_id: light.entity_id.clone(),
                kind: TopicKind::LightIsOn,
            },
        );
        if let Some(topic) = &light.brightness_state_topic {
            topics.state_topic.insert(
                topic.clone(),
                FieldMapping {
                    entity_id: light.entity_id.clone(),
                    kind: TopicKind::LightBrightness,
                },
            );
        }
        if let Some(topic) = &light.color_temp_state_topic {
            topics.state_topic.insert(
                topic.clone(),
                FieldMapping {
                    entity_id: light.entity_id.clone(),
                    kind: TopicKind::LightColorTemp,
                },
            );
        }
    }

    for sensor in &mqtt.sensor {
        topics.state_topic.insert(
            sensor.state_topic.clone(),
            FieldMapping {
                entity_id: sensor.entity_id.clone(),
                kind: TopicKind::SensorValue,
            },
        );
    }

    topics
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