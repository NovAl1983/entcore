use tokio::sync::mpsc;
use tokio::sync::oneshot;



use crate::message::{EntityMessage, UpdateSource};
use crate::entity::Entity;
use crate::state::{EntityState, FieldUpdate};

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
/// Обновить состояние конкретного поля сущности
     pub fn update_field(
    &self,
    entity_id: impl Into<String>,
    field: FieldUpdate,
    source: UpdateSource,
) {
    let _ = self.tx.send(EntityMessage::UpdateStateField {
        entity_id: entity_id.into(),
        field,
        source,
    });
}

  // блокирующий метод для сихронного кода  
  pub fn get_state_blocking(&self, entity_id: &str) -> Option<Entity> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(EntityMessage::GetState {
            entity_id: entity_id.to_string(),
            reply: tx,
        }).ok()?;
        rx.blocking_recv().ok()?
    }


// не блокирующий метод для async 
   pub async fn get_state(&self, entity_id: &str) -> Option<Entity> {
        let (tx, rx) = oneshot::channel();
        self.tx.send(EntityMessage::GetState {
            entity_id: entity_id.to_string(),
            reply: tx,
        }).ok()?;
        rx.await.ok()?
    }

     /// Подождать, пока менеджер обработает все текущие сообщения в очереди.
    /// Этот метод асинхронный, его можно вызывать в async-контексте через .await
    pub async fn wait_ready(&self) -> Result<(), String> {
        let (tx, rx) = oneshot::channel();
        
        // Отправляем сообщение в очередь менеджера
        if self.tx.send(EntityMessage::WaitReady { reply: tx }).is_err() {
            return Err("EntityManager уже остановлен".to_string());
        }
        
        // Ждем, пока менеджер обработает все сообщения перед нами и ответит в rx
        rx.await.map_err(|_| "Поток EntityManager аварийно завершился".to_string())
    }
    
  //  /// Прочитать сущность (async).
    // pub async fn get_state(&self, entity_id: &str) -> Option<Entity> { ... }
    
  //  /// Список всех (async).
    // pub async fn list_all(&self) -> Vec<Entity> { ... }
    
   // /// Завершить работу.
    // pub fn shutdown(&self) { ... }
}