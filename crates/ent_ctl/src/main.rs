use std::println;

use cfg::Config;
use ent_core::EntityManager;
use ent_mqtt::{mqtt_entities_from_config, MqttBridge};


#[tokio::main]
async  fn main() {
    println!("Запуск entcore...");

    // 1. Читаем конфиг
    let config = Config::load_config("config.toml").expect("Не удалось загрузить конфиг");
    println!("✅ Конфиг загружен");

    // 2. Создаём EntityManager
    let handle = EntityManager::spawn();
    println!("✅ EntityManager запущен");

    // 3. Конвертируем MQTT-сущности и Регистрируем их
    let mqtt_entities = mqtt_entities_from_config(&config.mqtt);
    println!("📦 Создано {} MQTT-сущностей", mqtt_entities.len()); 
    for entity in mqtt_entities {
        handle.register(entity);
    }

     println!("⏳ Ожидание обработки регистраций в EntityManager...");
    handle.wait_ready().await.expect("Не удалось дождаться инициализации EntityManager");
    println!("✅ EntityManager полностью инициализирован!");

    // 5. Проверяем: читаем одну сущность
    if let Some(entity) = handle.get_state("light.living_room").await {
        println!("✅ Прочитана: {} = {:?}", entity.id, entity.state);
    } else {
        eprintln!("⚠️ Сущность 'light.living_room' не найдена");
    }

     // 6. MQTT bridge
     let integrations = &config.integrations;
     let mqtt_config = integrations.mqtt.as_ref().expect("Секция [integrations.mqtt] обязательна");

     let mqtt_bridge = MqttBridge::new(mqtt_config, &config.mqtt, handle.clone());
     println!("✅ MqttBridge создан");

    //подписка на топики выполняется  в методе bridge.run()!!!!
    //  // 7. Подписка на один топик для теста (потом — на все)
    //  bridge.subscribe("/devices/wb-msw-v4_220/controls/Current Motion").await.expect("Не удалось подписаться");
    //  println!("✅ Подписка выполнена");

     // 8. Запускаем bridge в фоне + подписываемся на топики, main ждёт
    let bridge_handle = tokio::spawn(async move {
        mqtt_bridge.run().await;
    });

    println!("📡 MQTT bridge запущен. Ждём 5 секунд...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;



    // 9. Завершаемся
    println!("Завершение.");
    bridge_handle.abort();

}
