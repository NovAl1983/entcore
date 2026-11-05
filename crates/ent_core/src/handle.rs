use tokio::sync::oneshot;
use tokio::sync::mpsc::UnboundedSender;
use ent_types::{SystemMessage, Entity};



/// Пульт управления EntityManager.
/// Клонируется, раздаётся модулям.
#[derive(Clone)]
pub struct EntityHandle {
    tx_to_router: UnboundedSender<SystemMessage>,
}

impl EntityHandle {
    pub fn new(tx_to_router: UnboundedSender<SystemMessage>) -> Self {
        Self { tx_to_router }
    }

    /// Блокирующий метод для синхронного кода Lua.
    /// Он отправляет запрос в Роутер и останавливает поток Lua до получения ответа от Ядра.
    pub  fn get_state_blocking(&self, entity_id: &str) -> Option<Entity> {
        // Создаем одноразовый канал для получения ответа
        let (reply_tx, reply_rx) = oneshot::channel();

        // Упаковываем запрос в сообщение для Роутера
        let msg = SystemMessage::GetEntityState { 
            entity_id: entity_id.to_string(), 
            reply: reply_tx };

        // Отправляем сообщение на центральный Роутер
        if self.tx_to_router.send(msg).is_err() {
            return None;
        };

        // БЛОКИРУЕМ текущий поток ОС (в котором крутится Lua) до тех пор,
        // пока Ядро не пришлет нам ответ в reply_rx
        reply_rx.blocking_recv().ok()?
    }

    // // не блокирующий метод для async
    // pub async fn get_state(&self, entity_id: &str) -> Option<Entity> {
    //     let (tx, rx) = oneshot::channel();
    //     self.tx
    //         .send(EntityMessage::GetState {
    //             entity_id: entity_id.to_string(),
    //             reply: tx,
    //         })
    //         .ok()?;
    //     rx.await.ok()?
    // }




}
