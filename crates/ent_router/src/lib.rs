use ent_core::{Entity, FieldUpdate, UpdateSource};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;

/// Единый тип сообщений для всей системы умного дома.
/// Теперь ни один крейт не общается с другим напрямую — все создают эти сообщения 
/// и отправляют их в центральный Роутер.
#[derive(Debug)]
pub enum SystemMessage {
    // -----------------------------------------------------------------
    // КОМАНДЫ ДЛЯ ЯДРА (EntityManager)
    // -----------------------------------------------------------------
    
    /// Регистрирует новую сущность в базе данных ядра при старте модуля.
    RegisterEntity(Entity),

    /// Обновляет конкретное поле сущности (например, яркость или статус датчика).
    UpdateEntityField {
        entity_id: String,
        field: FieldUpdate,
        source: UpdateSource,
    },

    /// Этот вариант сообщения создается, когда кто-то (например, Lua) хочет узнать стейт сущности.
    GetEntityState {
        // Кого мы хотим прочитать (например, "light.living_room")
        entity_id: String,
        
        // Передаем передатчик (Sender) ОДНОРАЗОВОГО канала прямо внутри сообщения!
        // Тот, кто обработает этот запрос (Ядро), возьмет этот `reply`,
        // положит туда результат (Option<Entity>) и сделает .send().
        reply: oneshot::Sender<Option<Entity>>,
    },

    // -----------------------------------------------------------------
    // СОБЫТИЯ ДЛЯ СКРИПТОВ (ScriptRunner / Lua)
    // -----------------------------------------------------------------
    
    /// Сигнализирует движку скриптов, что состояние какой-то сущности изменилось,
    /// и нужно запустить соответствующие Lua-автоматизации.
    EntityChangedEvent {
        entity_id: String,
        // Мы передаем имя сущности, чтобы Lua понимала, на что именно реагировать
    },

    // -----------------------------------------------------------------
    // СИСТЕМНЫЕ СИГНАЛЫ
    // -----------------------------------------------------------------
    
    /// Команда на полную остановку всех фоновых потоков и тасок (плавное завершение).
    Shutdown,
}


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
