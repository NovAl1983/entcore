// ./crates/ent_types/src/lib.rs
use mlua::UserData;
use tokio::sync::oneshot;

// =====================================================================
// 1. ИСТОЧНИКИ ОБНОВЛЕНИЙ И СТРУКТУРЫ СОСТОЯНИЙ
// =====================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateSource {
    Mqtt,
    Script,
    Timer,
    Helper,
    System,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EntityState {
    Light(LightState),
    Sensor(SensorState),
    BinarySensor(BinarySensorState),
    Counter(CounterState),
    InputBoolean(InputBooleanState),
}

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

// =====================================================================
// 2. СУЩНОСТЬ (ENTITY)
// =====================================================================

#[derive(Debug, Clone, PartialEq)]
pub struct Entity {
    pub id: String,
    pub friendly_name: String,
    pub state: EntityState,
}

// =====================================================================
// 3. ЦЕНТРАЛЬНЫЙ enum СООБЩЕНИЙ (SYSTEM MESSAGE)
// =====================================================================

#[derive(Debug)]
pub enum SystemMessage {
    /// Регистрирует новую сущность в базе данных ядра при старте модуля.
    RegisterEntity(Entity),

    /// Обновляет конкретное поле сущности.
    UpdateEntityField {
        entity_id: String,
        field: FieldUpdate,
        source: UpdateSource,
    },

    /// Запрос состояния сущности (например, из Lua) с одноразовым каналом ответа.
    GetEntityState {
        entity_id: String,
        reply: oneshot::Sender<Option<Entity>>,
    },

    /// Сигнализирует движку скриптов, что состояние сущности изменилось.
    EntityChangedEvent {
        entity_id: String,
    },

    /// Команда на плавную остановку всех фоновых потоков и тасок.
    Shutdown,
}
