use std::println;
use std::thread;
use tokio::sync::mpsc;

use cfg::Config;
use ent_core::EntityManager;
use ent_core::handle::EntityHandle;
use ent_lua::runner::ScriptRunner;
use ent_mqtt::{MqttBridge, mqtt_entities_from_config};
use ent_router::Router;
use ent_types::SystemMessage;

#[tokio::main]
async fn main() {
    println!("=== [entcore] Запуск системы умного дома (Топология Звезда) ===");

    // 1. Читаем конфиг (твоя оригинальная логика)
    let config = Config::load_config("config.toml").expect("Не удалось загрузить конфиг");
    println!("✅ Конфиг загружен");

    // 2. Создаем каналы обратной связи (рельсы ОТ Роутера К модулям)
    let (tx_to_core, rx_for_core) = mpsc::unbounded_channel::<SystemMessage>();
    let (tx_to_lua, rx_for_lua) = mpsc::unbounded_channel::<SystemMessage>();
    let (tx_to_mqtt, _rx_for_mqtt) = mpsc::unbounded_channel::<SystemMessage>();

    // 3. Инициализируем Роутер
    // Получаем сам роутер и входной tx_in, через который все модули будут слать сообщения
    let (router, tx_in) = Router::new(tx_to_core, tx_to_lua, tx_to_mqtt);

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

    // 5. Запускаем EntityManager (Ядро)
    // Оно будет слушать свою ветку rx_for_core и слать ответы/события обратно в Роутер через tx_in
    EntityManager::spawn(rx_for_core, tx_in.clone());
    println!("✅ EntityManager (Ядро) успешно запущен.");

    // 6. Конвертируем MQTT-сущности из конфига и регистрируем их в Системе
    let mqtt_entities = mqtt_entities_from_config(&config.mqtt);
    println!(
        "📦 Обнаружено {} MQTT-сущностей в конфиге. Регистрируем...",
        mqtt_entities.len()
    );
    for entity in mqtt_entities {
        tx_in
            .send(SystemMessage::RegisterEntity(entity))
            .expect("Не удалось отправить команду регистрации сущности");
    }

    // 7. Создаем EntityHandle для Lua, который смотрит на Роутер
    let lua_handle = EntityHandle::new(tx_in.clone());

    //TODO нужно реализовать способ дожидаться пока все регистрации обработаются перед запуском ScriptRunner

    // 8. Запуск движка автоматизации Lua ScriptRunner
    ScriptRunner::spawn(
        config.system.scripts_directory.clone(),
        rx_for_lua,
        lua_handle,
    );
    println!("✅ Движок автоматизации ScriptRunner (Lua) запущен.");

    // 9. Запуск MQTT-моста
    let integrations = &config.integrations;
    let mqtt_config = integrations
        .mqtt
        .as_ref()
        .expect("Секция [integrations.mqtt] обязательна");

    // Передаем мосту клон tx_in, чтобы он слал FieldUpdate сообщения на Роутер
    let mqtt_bridge = MqttBridge::new(mqtt_config, &config.mqtt, tx_in.clone());

    tokio::spawn(async move {
        mqtt_bridge.run().await;
    });
    println!("✅ Мост MQTT запущен и подключается к брокеру.");

    // 10. Основной рабочий цикл ctl-модуля (MVP работает 10 секунд и изящно завершается)
    println!("\n📡 Система полностью активна и обрабатывает события. Ждем 10 секунд...");
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    println!("\n🛑 Инициировано плановое завершение работы системы...");
    let _ = tx_in.send(SystemMessage::Shutdown);

    // Микро-пауза, чтобы потоки успели напечатать логи закрытия
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    println!("=== [entcore] Работа завершена ===");
}
