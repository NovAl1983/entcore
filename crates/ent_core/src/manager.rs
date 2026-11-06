// ./crates/ent_core/src/manager.rs
use std::collections::HashMap;
use std::{eprintln, println, thread};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

// Импортируем всё необходимое из единого центра типов
use ent_types::{Entity, EntityState, FieldUpdate, SystemMessage};

pub struct EntityManager {
    // приемник от Роутера
    rx: UnboundedReceiver<SystemMessage>,
    // передатчик в Роутер (чтобы кричать о событиях)
    tx_to_router: UnboundedSender<SystemMessage>,

    // внутренняя база данных сущностей
    pub entities: HashMap<String, Entity>,
}

impl EntityManager {
    /// Спавнит менеджер в отдельном потоке ОС
    /// Теперь на вход принимаем наш rx_for_core и клон tx_in из main.rs
    pub fn spawn(
        rx: UnboundedReceiver<SystemMessage>,
        tx_to_router: UnboundedSender<SystemMessage>,
    ) {
        let mut manager = Self {
            rx,
            tx_to_router,
            entities: HashMap::new(),
        };

        // Оставляем менеджер в собственном потоке ОС
        thread::Builder::new()
            .name("[entcore] entity-manager".into())
            .spawn(move || {
                manager.run();
            })
            .expect("Не удалось запустить поток EntityManager");
    }

    /// Главный асинхронный цикл обработки данных ядра
    fn run(&mut self) {
        println!("[Ядро] Менеджер сущностей запущен.");

        while let Some(msg) = self.rx.blocking_recv() {
            match msg {
                // 1. РЕГИСТРАЦИЯ СУЩНОСТИ
                SystemMessage::RegisterEntity(entity) => {
                    let id = entity.id.clone();
                    if self.entities.contains_key(&id) {
                        println!("[Ядро] Сущность с id = {} уже существует", id);
                        continue;
                    }

                    self.entities.insert(id.clone(), entity);
                    let event = SystemMessage::EntityChangedEvent { entity_id: id.clone() };
                    let _ = self.tx_to_router.send(event);
                    println!("[Ядро] Сущность с id = {} успешно зарегистрирована", id);
                }

                // 2. ОБНОВЛЕНИЕ КОНКРЕТНОГО ПОЛЯ
                SystemMessage::UpdateEntityField {
                    entity_id,
                    field,
                    source,
                } => {
                    if let Some(entity) = self.entities.get_mut(&entity_id) {
                        // use crate::state::{EntityState, FieldUpdate::*};

                        // Вычисляем, изменилось ли значение на самом деле (защита от дедупликации)
                        let changed = match (&mut entity.state, field) {
                            (EntityState::Light(light), FieldUpdate::IsOn(v)) => {
                                let old = light.is_on;
                                light.is_on = v;
                                old != v
                            }
                            (EntityState::Light(light), FieldUpdate::Brightness(v)) => {
                                let old = light.brightness;
                                light.brightness = v;
                                old != v
                            }
                            (EntityState::Light(light), FieldUpdate::ColorTemp(v)) => {
                                let old = light.color_temp;
                                light.color_temp = v;
                                old != v
                            }
                            (EntityState::Sensor(sensor), FieldUpdate::Value(v)) => {
                                let old = sensor.value;
                                sensor.value = v;
                                old != v
                            }
                            (EntityState::BinarySensor(binary), FieldUpdate::IsOn(v)) => {
                                let old = binary.is_on;
                                binary.is_on = v;
                                old != v
                            }
                            (EntityState::InputBoolean(input_bool), FieldUpdate::IsOn(v)) => {
                                let old = input_bool.is_on;
                                input_bool.is_on = v;
                                old != v
                            }
                            _ => {
                                eprintln!("[Ядро] ⚠️ Несовместимое поле для '{}'", entity_id);
                                false
                            }
                        };

                        // Если значение НЕ изменилось — срабатывает дедупликация и переходим к следующему сообщению.
                        if !changed {
                            // Значение то же самое, защитили систему от лишней работы!
                            continue;
                        }

                        // Сюда код дойдет ТОЛЬКО если значение реально обновилось
                        println!("[Ядро] 🔄 '{}' обновлена (source: {:?})", entity_id, source);

                        // Отправляем "будильник" в Роутер для пробуждения Lua-скриптов
                        let event = SystemMessage::EntityChangedEvent {
                            entity_id: entity_id.clone(),
                        };
                        let _ = self.tx_to_router.send(event);
                    } else {
                        // Срабатывает, только если id сущности вообще нет в базе данных ядра!
                        eprintln!(
                            "[Ядро] ⚠️ UpdateEntityField: неизвестная сущность '{}'",
                            entity_id
                        );
                    }
                }

                // 3. ОБРАБОТКА ЗАПРОСА СОСТОЯНИЯ
                SystemMessage::GetEntityState { entity_id, reply } => {
                    let entity_clone = self.entities.get(&entity_id).cloned();
                    // Отправляем ответ НАПРЯМУЮ в вложенный конверт (минуя Роутер)
                    let _ = reply.send(entity_clone);
                }

                 // 5. БАРЬЕР ГОТОВНОСТИ
                SystemMessage::WaitReady { reply } => {
                    // Отправляем сигнал готовности обратно в main
                let _ = reply.send(());
                }

                // 4. ПЛАВНОЕ ЗАВЕРШЕНИЕ
                SystemMessage::Shutdown => {
                    println!("[Ядро] Получен сигнал Shutdown. Остановка...");
                    break;
                }

                // Все остальные сообщения (например, события для Lua) Ядро просто игнорирует
                _ => {}
            }
        }
        println!("[Ядро] Менеджер сущностей успешно остановлен.");
    }
}
