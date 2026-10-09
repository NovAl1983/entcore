use crate::entity::Entity;
use crate::state::{EntityState, FieldUpdate};
use tokio::sync::oneshot;

/// Сообщения, которые модули шлют в EntityManager.
#[derive(Debug)]
pub enum EntityMessage {
    /// Зарегистрировать сущность при старте.
    Register(Entity),

    /// Обновить состояние сущности.
    UpdateState {
        entity_id: String,
        new_state: EntityState,
        source: UpdateSource,
    },

    UpdateStateField {
    entity_id: String,
    field: FieldUpdate,
    source: UpdateSource,
},

    /// Прочитать сущность по id. Ответ — через oneshot.
    GetState {
        entity_id: String,
        reply: oneshot::Sender<Option<Entity>>,
    },


    /// Получить список всех сущностей. Для отладки и WebUI.
    ListAll {
        reply: oneshot::Sender<Vec<Entity>>,
    },

    /// Завершить работу актора.
    Shutdown,
}

/// Кто инициировал изменение.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateSource {
    Mqtt,
    Script,
    Timer,
    Helper,
    System,
}