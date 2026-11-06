// ./crates/ent_router/src/lib.rs
use ent_types::SystemMessage;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub struct Router {
    rx_in: UnboundedReceiver<SystemMessage>,
    _tx_in: UnboundedSender<SystemMessage>, // добавили _
    tx_to_core: UnboundedSender<SystemMessage>,
    tx_to_lua: UnboundedSender<SystemMessage>,
    _tx_to_mqtt: UnboundedSender<SystemMessage>, // добавили _
}

impl Router {
    pub fn new(
        tx_to_core: UnboundedSender<SystemMessage>,
        tx_to_lua: UnboundedSender<SystemMessage>,
        tx_to_mqtt: UnboundedSender<SystemMessage>,
    ) -> (Self, UnboundedSender<SystemMessage>) {
        let (tx_in, rx_in) = tokio::sync::mpsc::unbounded_channel();

        let router = Self {
            rx_in,
            _tx_in: tx_in.clone(),
            tx_to_core,
            tx_to_lua,
            _tx_to_mqtt: tx_to_mqtt,
        };

        (router, tx_in)
    }

    pub async fn run(mut self) {
        println!("[Роутер] Центральный маршрутизатор запущен.");

        while let Some(msg) = self.rx_in.recv().await {
            match msg {
                SystemMessage::RegisterEntity(_)
                | SystemMessage::UpdateEntityField { .. }
                | SystemMessage::GetEntityState { .. }
                | SystemMessage::WaitReady { .. } => {
                    let _ = self.tx_to_core.send(msg);
                }
                SystemMessage::EntityChangedEvent { .. } => {
                    let _ = self.tx_to_lua.send(msg);
                }
                SystemMessage::Shutdown => {
                    println!("[Роутер] Получен сигнал Shutdown. Останавливаем модули...");
                    let _ = self.tx_to_core.send(SystemMessage::Shutdown);
                    let _ = self.tx_to_lua.send(SystemMessage::Shutdown);
                    break;
                }
            }
        }
        println!("[Роутер] Маршрутизатор успешно остановлен.");
    }
}
