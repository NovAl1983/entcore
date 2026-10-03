use std::println;

use cfg::Config;
use ent_core::EntityManager;
use ent_mqtt::converter::mqtt_entities_from_config;

fn main() {
    println!("Запуск entcore...");

    // 1. Читаем конфиг
    let config = Config::load_config("config.toml").expect("Не удалось загрузить конфиг");
    println!("✅ Конфиг загружен");

    // 2. Создаём EntityManager
    let handle = EntityManager::spawn();
    println!("✅ EntityManager запущен");

    // 3. Конвертируем MQTT-сущности
    let mqtt_entities = mqtt_entities_from_config(&config.mqtt);
    println!("📦 Создано {} MQTT-сущностей", mqtt_entities.len());

    // 4. Регистрируем их
    for entity in mqtt_entities {
        handle.register(entity);
    }

    // 5. Пауза — дать менеджеру обработать (пока костыль)
    std::thread::sleep(std::time::Duration::from_millis(50));

    // 6. Проверяем: читаем одну сущность
    if let Some(entity) = handle.get_state_blocking("light.living_room") {
        println!("✅ Прочитана: {} = {:?}", entity.id, entity.state);
    } else {
        eprintln!("⚠️ Сущность 'light.living_room' не найдена");
    }

    // 7. Завершаемся
    println!("Готово.");

}
