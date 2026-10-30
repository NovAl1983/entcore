use mlua::UserData;



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

#[derive(Debug, Clone, PartialEq, UserData)]
pub struct LightState {
    pub is_on: bool,
    pub brightness: Option<u8>,
    pub color_temp: Option<u16>,
}







#[derive(Debug, Clone, PartialEq)]
pub struct SensorState {
    pub value: f64,
}





#[derive(Debug, Clone, PartialEq)]
pub struct BinarySensorState {
    pub is_on: bool,
}





#[derive(Debug, Clone, PartialEq)]
pub struct CounterState {
    pub value: u64,
}






#[derive(Debug, Clone, PartialEq)]
pub struct InputBooleanState {
    pub is_on: bool,
}






#[derive(Debug, Clone, PartialEq)]
pub enum FieldUpdate {
    IsOn(bool),
    Brightness(Option<u8>),
    ColorTemp(Option<u16>),
    Value(f64),
}

