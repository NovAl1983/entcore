
//cd /d/Rust/projects/entcore/config.toml
//D:\Rust\projects\entcore\config.toml


//use std::{collections::HashMap, println};
use serde::Deserialize;
use std::error::Error;



#[derive(Debug, Deserialize, Clone)]
pub struct Config {
pub system: System,
pub integrations: Integrations,
#[serde(default)]
pub mqtt: Mqtt,
#[serde(default)]
pub http: Vec<Http>,
#[serde(default)]
pub helper: Helper,
}



impl Config {
    pub fn load_config(path: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let cfg_str = std::fs::read_to_string(path)?;
        let config: Config = toml::from_str(&cfg_str)?;
        
        let has_integration = config.integrations.mqtt.is_some()
            || config.integrations.modbus_rtu.is_some();
        let has_helper = !config.helper.input_boolean.is_empty();
        
        if !has_integration && !has_helper {
            return Err(format!(
                "Конфиг {path} пуст: нет ни интеграций (mqtt/modbus_rtu), \
                 ни helpers (input_boolean). Движку не с чем работать."
            ).into());
        }
        
        Ok(config)
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct System {
    pub scripts_directory: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Integrations {
    pub mqtt: Option<MqttBroker>,
    pub modbus_rtu:Option<ModbusRtu>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MqttBroker {
    pub host: String,
    pub  port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModbusRtu {
    pub port: String, // "/dev/ttyRS485-1"
    pub baud_rate: u32, //  9600
    pub data_bits: u8, // 8
    pub stop_bits: u8, // 2
    // TODO: заменить на enum Parity
    pub parity: String, // "N"
}


#[derive(Debug, Deserialize, Clone, Default)]
pub struct Mqtt {
    #[serde(default)]
    pub light: Vec<MqttLight>,
    #[serde(default)]
    pub sensor: Vec<MqttSensor>,

}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MqttLight {
    pub entity_id: String,
    pub name: String,
    pub state_topic: String,
    pub command_topic: String,
    pub brightness_state_topic: Option<String>,
    pub brightness_command_topic: Option<String>,
    pub brightness_scale: Option<u16>,
    pub color_temp_state_topic: Option<String>,
    pub color_temp_command_topic: Option<String>,
    pub color_temp_scale: Option<u16>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct MqttSensor {
    pub entity_id: String,
    pub name: String,
    pub state_topic: String
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Http {
    pub entity_id: String,
    pub name: String,
    pub url: String,
    pub scan_interval: u64,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Helper {
    #[serde(default)]
    pub input_boolean: Vec<HelperInputBoolean>,
}


#[derive(Debug, Deserialize, Clone, Default)]
pub struct HelperInputBoolean {
    pub entity_id: String,
    pub name: String,
    pub initial: bool,
}

#[test]
fn test_load_full_config() {
    let config = Config::load_config("../../config.toml")
        .expect("Не удалось загрузить конфиг");
    println!("{:#?}", config);
    
    // MQTT — обязательная секция в тестовом конфиге
    let mqtt = config.integrations.mqtt.as_ref()
        .expect("Ожидалась секция [integrations.mqtt]");
    assert_eq!(mqtt.host, "localhost");
    assert_eq!(mqtt.port, 1883);
    
    // Modbus — тоже есть в тестовом конфиге
    let modbus = config.integrations.modbus_rtu.as_ref()
        .expect("Ожидалась секция [integrations.modbus_rtu]");
    assert_eq!(modbus.port, "/dev/ttyRS485-1");
    
    // Сущности
    assert_eq!(config.mqtt.light.len(), 1);
    assert_eq!(config.mqtt.light[0].entity_id, "light.living_room");
    assert_eq!(config.mqtt.sensor.len(), 2);
    assert_eq!(config.http.len(), 1);
    assert_eq!(config.helper.input_boolean.len(), 1);
}