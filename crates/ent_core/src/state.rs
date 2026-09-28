

#[derive(Debug, Clone, PartialEq)]
pub enum EntityState {
    Light {
        is_on: bool,
        brightness: Option<u8>,
        color_temp: Option<u16>,
    },
    Sensor {value: f64},
    BinarySensor {is_on: bool},
    Counter {value: u64},
    InputBoolean {is_on: bool},
}

