# entcore - архитектурные решения

## Версия 1 (2026-09-28)
📌 Договорённости (v1)
Архитектура
Парадигма «Всё есть сущности» (HA-style).

Entity — типизированный объект: id, friendly_name, state.

EntityState — enum: BinarySensor, Sensor, Light, InputBoolean.

Домены фиксированы: light, sensor, binary_sensor, input_boolean.

entity_id формат: <domain>.<object_id>, например light.living_room.

EntityManager — актор в отдельном std::thread.

Владеет HashMap<String, Entity>.

Общается через каналы (mpsc + oneshot).

Никаких RwLock/Mutex на базе.

Синхронный цикл blocking_recv.

Сущности создаются в модулях своих протоколов (Вариант A).

ent_mqtt: читает config.mqtt, создаёт Entity, регистрирует.

ent_modbus: читает свою секцию, создаёт Entity, регистрирует.

ent_helper: создаёт виртуальные сущности.

ent_core не знает про Config.

EntityManager принимает готовые Entity.

Порядок регистрации — в main (ent_ctl), детерминированный.

text
let mqtt_entities   = mqtt::register(&config.mqtt, &handle)?;
let http_entities   = http::register(&config.http, &handle)?;
let helper_entities = helper::register(&config.helper, &handle)?;
Модули — отдельные крейты.

ent_core — ядро (без I/O, без serde).

cfg — парсинг TOML.

ent_ctl — main + запуск.

ent_mqtt, ent_runes, ent_timer, ent_modbus — позже.

Модель данных
EntityState — минимальная, без Switch, Counter, Event.

rust
pub enum EntityState {
    BinarySensor { is_on: bool },
    Sensor { value: f64 },
    Light {
        is_on: bool,
        brightness: Option<u8>,
        color_temp: Option<u16>,
    },
    InputBoolean { is_on: bool },
}
Switch, Counter, Event — позже, когда понадобятся.

Счётчики нажатий WB — позже (модель не решена).

Serde — только на границе (cfg).

EntityState — без serde.

MqttLight, MqttSensor — с serde.

PartialEq на EntityState и Entity — для дедупликации в EntityManager.

Конфиг
Option — только когда «отсутствие» семантически важно.

Integrations.mqtt: Option<MqttBroker> — да.

Mqtt.light: Vec<MqttLight> + #[serde(default)] — да.

Option<Vec<T>> — никогда.

EntityState не конвертируется из TOML напрямую.

TOML → MqttLight (serde).

MqttLight → Entity (код в ent_mqtt).

Валидация
Валидация — TODO. Позже решим:

Уровень 1 (cfg): синтаксис, формат entity_id, уникальность. Без знания модели.

Уровень 2 (ent_core): семантика — домен ↔ EntityState.

Уровень 3 (отдельный модуль): сквозная проверка.

Возможный вариант: cfg собирает все entity_id в мапу и проверяет уникальность.

Решение отложено.

Рантайм
Перезапуск движка — ручной (systemctl restart). Без hot-reload.

При ошибке конфига — паника/exit(1), не откат. Лучше упасть громко.

Скрипты — stateless. Vm создаётся на событие, уничтожается. main() — чистая, только возвращает правила. Логика — в then-функциях.

Вызов скриптов — по func_hash, не весь скрипт.

Потоки (MVP):

EntityManager — 1 std::thread.

ScriptRunner — 1 std::thread (позже — пул).

Timer — 1 std::thread.

MqttBridge — tokio async (не std::thread).

## Правило: минимум tokio

- `ent_core`: `tokio = { features = ["sync"] }` — только mpsc/oneshot.
- `ent_mqtt`, `ent_ctl`: полный tokio (rt, net, macros).
- `ent_runes`, `ent_timer`, `ent_modbus`: только `std` + `tokio::sync` при необходимости.
- По умолчанию — `std`. Tokio — исключение.