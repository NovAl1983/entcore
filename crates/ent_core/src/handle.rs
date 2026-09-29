use tokio::sync::mpsc;

use crate::message::{EntityMessage, UpdateSource};
use crate::entity::Entity;
use crate::state::EntityState;

/// Пульт управления EntityManager.
/// Клонируется, раздаётся модулям.
#[derive(Clone)]
pub struct EntityHandle {
    tx: mpsc::UnboundedSender<EntityMessage>,
}

impl EntityHandle {
    pub(crate) fn new(tx: mpsc::UnboundedSender<EntityMessage>) -> Self { 
        Self {tx}
     }
    
    /// Зарегистрировать сущность. Fire-and-forget.
    pub fn register(&self, entity: Entity) {
let _ = self.tx.send(EntityMessage::Register(entity));

    }

    
    /// Обновить состояние. Fire-and-forget.
    pub fn update_state(
        &self,
        entity_id: impl Into<String>,
        new_state: EntityState,
        source: UpdateSource,
    ) { 
        let _ = self.tx.send(EntityMessage::UpdateState { 
                entity_id: entity_id.into(), 
                new_state, 
                source 
            });
     }
    
  //  /// Прочитать сущность (sync).
    // pub fn get_state_blocking(&self, entity_id: &str) -> Option<Entity> { ... }
    
  //  /// Прочитать сущность (async).
    // pub async fn get_state(&self, entity_id: &str) -> Option<Entity> { ... }
    
  //  /// Список всех (async).
    // pub async fn list_all(&self) -> Vec<Entity> { ... }
    
   // /// Завершить работу.
    // pub fn shutdown(&self) { ... }
}