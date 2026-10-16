
use rune_macros::Any;


// Enum теперь — это просто чистый диспетчер, который объединяет эти структуры
#[derive(Debug, Clone, PartialEq)]
pub enum EntityState {
    Light(LightState),
    Sensor(SensorState),
    BinarySensor(BinarySensorState),
    Counter(CounterState),
    InputBoolean(InputBooleanState),
}

// Каждая сущность теперь имеет свою независимую, чистую структуру данных

#[derive(Debug, Clone, PartialEq, Any)]
pub struct LightState {
    #[rune(get)]
    pub is_on: bool,
    #[rune(get)]
    pub brightness: Option<u8>,
    #[rune(get)]
    pub color_temp: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Any)]
pub struct SensorState {
    #[rune(get)]
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Any)]
pub struct BinarySensorState {
    #[rune(get)]
    pub is_on: bool,
}

#[derive(Debug, Clone, PartialEq, Any)]
pub struct CounterState {
    #[rune(get)]
    pub value: u64,
}

#[derive(Debug, Clone, PartialEq, Any)]
pub struct InputBooleanState {
    #[rune(get)]
    pub is_on: bool,
}




#[derive(Debug, Clone, PartialEq)]
pub enum FieldUpdate {
    IsOn(bool),
    Brightness(Option<u8>),
    ColorTemp(Option<u16>),
    Value(f64),
}

