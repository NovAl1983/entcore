use std::println;
use std::thread;
use tokio::sync::mpsc;

use cfg::Config;
use ent_mqtt::{MqttBridge, mqtt_entities_from_config};
use ent_router::{Router, SystemMessage};

#[tokio::main]
async fn main() {
    println!("Запуск entcore с центральным маршрутизатором...");

    // 1. Читаем конфиг (твоя оригинальная логика)
    let config = Config::load_config("config.toml").expect("Не удалось загрузить конфиг");
    println!("✅ Конфиг загружен");

    // 2. Создаем каналы обратной связи (рельсы ОТ Роутера К модулям)
    let (tx_to_core, rx_for_core) = mpsc::unbounded_channel::<SystemMessage>();
    let (tx_to_lua, rx_for_lua) = mpsc::unbounded_channel::<SystemMessage>();
    let (tx_to_mqtt, rx_for_mqtt) = mpsc::unbounded_channel::<SystemMessage>();

    // 3. Инициализируем Роутер
    // Получаем сам роутер и входной tx_in, через который все модули будут слать сообщения
    let (router, tx_in) = Router::new(tx_to_core, tx_to_lua, tx_to_mqtt);

    // Клонируем входной передатчик для MQTT-моста
    let tx_for_core = tx_in.clone();
    let tx_for_lua = tx_in.clone();
    let tx_for_mqtt = tx_in.clone();

    // 4. Выделяем под Роутер отдельный поток операционной системы
    thread::Builder::new()
        .name("[entcore] system-router".into())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("Не удалось запустить рантайм для Роутера");

            rt.block_on(router.run());
        })
        .expect("Не удалось запустить поток Роутера");

    println!("✅ Поток центрального Роутера запущен");

    // -----------------------------------------------------------------
    // ЗАДЕЛ НА СЛЕДУЮЩИЕ ШАГИ (Пока закомментировано, так как мы еще не переписали эти крейты)
    // -----------------------------------------------------------------
    
    // 5. Создаём EntityManager (он должен будет слушать rx_for_core)
    // let handle = EntityManager::spawn(rx_for_core, tx_in.clone());
    // println!("✅ EntityManager запущен");

    // 6. Конвертируем MQTT-сущности и Регистрируем их через Роутер
    // let mqtt_entities = mqtt_entities_from_config(&config.mqtt);
    // println!("📦 Создано {} MQTT-сущностей", mqtt_entities.len());
    // for entity in mqtt_entities {
    //     tx_in.send(SystemMessage::RegisterEntity(entity)).unwrap();
    // }

    // 7. Запуск Lua ScriptRunner (он должен будет слушать rx_for_lua)
    // let lua_runner = ScriptRunner::spawn(config.system.scripts_directory, rx_for_lua, tx_in.clone());

    // 8. Запуск MQTT bridge (он будет слать сообщения в tx_for_mqtt)
    // let integrations = &config.integrations;
    // let mqtt_config = integrations.mqtt.as_ref().expect("Секция [integrations.mqtt] обязательна");
    // let mqtt_bridge = MqttBridge::new(mqtt_config, &config.mqtt, tx_for_mqtt);
    // let bridge_handle = tokio::spawn(async move {
    //     mqtt_bridge.run().await;
    // });

    // Временная заглушка для теста: спим 5 секунд и завершаем работу
    println!("📡 Система активна. Работаем 5 секунд...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    println!("Завершение работы...");
    let _ = tx_in.send(SystemMessage::Shutdown);
    
    // Даем микро-паузу, чтобы потоки успели напечатать свои логи закрытия
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}
