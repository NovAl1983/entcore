use ent_types::SystemMessage;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;




pub struct Router {
    // Входной канал: сюда будут писать ВСЕ крейты
    rx_in: UnboundedReceiver<SystemMessage>,
    // Этот передатчик мы будем клонировать и раздавать модулям
    tx_in: UnboundedSender<SystemMessage>,

    // Выходные каналы для обратной связи (наши «рельсы» к модулям)
    tx_to_core: UnboundedSender<SystemMessage>,
    tx_to_lua: UnboundedSender<SystemMessage>,
    tx_to_mqtt: UnboundedSender<SystemMessage>,
}

impl Router {
    /// Создает роутер и инициализирует все каналы связи
    pub fn new(
        tx_to_core: UnboundedSender<SystemMessage>,
        tx_to_lua: UnboundedSender<SystemMessage>,
        tx_to_mqtt: UnboundedSender<SystemMessage>,
    ) -> (Self, UnboundedSender<SystemMessage>) {
        // Создаем центральный входной канал (unbounded, чтобы не блокировать отправителей)
        let (tx_in, rx_in) = tokio::sync::mpsc::unbounded_channel();

        let router = Self {
            rx_in,
            tx_in: tx_in.clone(),
            tx_to_core,
            tx_to_lua,
            tx_to_mqtt,
        };

        // Возвращаем сам роутер и его входной tx_in, который main.rs раздаст всем
        (router, tx_in)
    }

    /// Главный асинхронный цикл распределения сообщений
    pub async fn run(mut self) {
        println!("[Роутер] Центральный маршрутизатор запущен.");
        
        while let Some(msg) = self.rx_in.recv().await {
            match msg {
                // Если это регистрация, обновление или ЗАПРОС состояния от Lua — 
                // Роутер просто перекладывает это в канал Ядра
                SystemMessage::RegisterEntity(_) 
                | SystemMessage::UpdateEntityField { .. } 
                | SystemMessage::GetEntityState { .. } => { // <-- Добавили наш новый запрос
                    let _ = self.tx_to_core.send(msg);
                }
                // Если это событие для скриптов — шлем строго в Lua
                SystemMessage::EntityChangedEvent { .. } => {
                    let _ = self.tx_to_lua.send(msg);
                }
                // Если пришел сигнал остановки — рассылаем его всем и выходим из цикла
                SystemMessage::Shutdown => {
                    println!("[Роутер] Получен сигнал Shutdown. Останавливаем модули...");
                    let _ = self.tx_to_core.send(SystemMessage::Shutdown);
                    let _ = self.tx_to_lua.send(SystemMessage::Shutdown);
                    // Когда появится логика в MQTT, раскомментируем строчку ниже:
                    // let _ = self.tx_to_mqtt.send(SystemMessage::Shutdown);
                    break;
                }
            }
        }
        println!("[Роутер] Маршрутизатор успешно остановлен.");
    }
}
