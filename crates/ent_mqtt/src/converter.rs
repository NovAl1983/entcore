
use std::collections::HashMap;

use cfg::Mqtt;
use ent_core::{Entity, EntityState};


pub struct TopicInfo {
    pub id: String,
    // pub state: EntityState,
    pub fleid_state: String,
}

impl  TopicInfo {
    pub fn new(id: String, fleid_state: String) -> Self {
        Self { id, fleid_state }

    }
    
}

pub struct Topics {
    pub state_topic: HashMap<String, TopicInfo>, // topic->TopicInfo
    pub command_topic: HashMap<TopicInfo, String> // TopicInfo->topic
}

impl Topics {
    pub fn new() -> Self {
        Self { state_topic: HashMap::new(), command_topic: HashMap::new() }
    }
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
    let mut topics = Topics::new();

    for item in &mqtt.light {
        topics.state_topic.insert(item.state_topic.clone(), TopicInfo { id: item.entity_id.clone(), fleid_state: "is_on".to_string() });

        if let Some(b) = &item.brightness_state_topic {
            topics.state_topic.insert(b.clone(), TopicInfo { id: item.entity_id.clone(), fleid_state: "brightness".to_string() });
        }

        if let Some(ct) = &item.color_temp_state_topic {
            topics.state_topic.insert(ct.clone(), TopicInfo { id: item.entity_id.clone(), fleid_state: "color_temp".to_string() });
        }
    }

    for item in &mqtt.sensor {
         topics.state_topic.insert(item.state_topic.clone(), TopicInfo { id: item.entity_id.clone(), fleid_state: "value".to_string() });
    }


    topics
    
}



//  mqtt: Mqtt {
//         light: [
//             MqttLight {
//                 entity_id: "light.living_room",
//                 name: "люстра в зале",
//                 state_topic: "/devices/wb-mr6cv3_189/controls/K2",
//                 command_topic: "/devices/wb-mr6cv3_189/controls/K2/on",
//                 brightness_state_topic: Some(
//                     "/devices/wb-mr6cv3_189/controls/Brightness",
//                 ),
//                 brightness_command_topic: Some(
//                     "/devices/wb-mr6cv3_189/controls/Brightness/on",
//                 ),
//                 brightness_scale: Some(
//                     254,
//                 ),
//                 color_temp_state_topic: None,
//                 color_temp_command_topic: None,
//                 color_temp_scale: None,
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