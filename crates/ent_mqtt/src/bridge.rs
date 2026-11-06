use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};
use std::println;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;

use crate::converter::{TopicKind, Topics, mqtt_from_config};
use cfg::{Mqtt, MqttBroker};

use ent_types::{FieldUpdate, SystemMessage, UpdateSource};

pub struct MqttBridge {
    client: AsyncClient,
    eventloop: rumqttc::EventLoop,
    topics: Topics,
    tx_to_router: UnboundedSender<SystemMessage>,
}

impl MqttBridge {
    pub fn new(
        broker: &MqttBroker,
        mqtt: &Mqtt,
        tx_to_router: UnboundedSender<SystemMessage>,
    ) -> Self {
        let mut options = MqttOptions::new("entcore", &broker.host, broker.port);
        options.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(options, 50);

        let topics = mqtt_from_config(mqtt);

        Self {
            client,
            eventloop,
            topics,
            tx_to_router,
        }
    }

    pub async fn run(mut self) {
        let mut subscribe: Vec<String> = Vec::new();
        loop {
            match self.eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    let payload = String::from_utf8_lossy(&p.payload);
                    if let Some(mapping) = self.topics.state_topic.get(&p.topic) {
                        // Парсим сырые данные из MQTT в наши типизированные FieldUpdate
                        let field = match mapping.kind {
                            TopicKind::LightIsOn => {
                                let v = payload == "1" || payload == "true" || payload == "ON";
                                FieldUpdate::IsOn(v)
                            }
                            TopicKind::LightBrightness => {
                                FieldUpdate::Brightness(payload.parse().ok())
                            }
                            TopicKind::LightColorTemp => {
                                FieldUpdate::ColorTemp(payload.parse().ok())
                            }
                            TopicKind::SensorValue => {
                                FieldUpdate::Value(payload.parse().unwrap_or(0.0))
                            }
                        };
                        // Создаем сообщение Роутеру!
                        let msg = SystemMessage::UpdateEntityField {
                            entity_id: mapping.entity_id.clone(),
                            field,
                            source: UpdateSource::Mqtt,
                        };

                        // Отправляем на центральный Роутер. Если роутер упал, .is_err() это отловит.
                        if self.tx_to_router.send(msg).is_err() {
                            println!("⚠️ [MQTT Мост] Центральный роутер остановился. Выходим.");
                            break;
                        }
                    }
                }
                Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                    println!("✅ MQTT: ConnAck подключено к брокеру");
                    subscribe.clear();
                    for topic in self.topics.state_topic.keys() {
                        let t = topic.to_string();
                        println!("Топик {} добавлен в вектор", &t);
                        subscribe.push(t);
                    }
                    if let Some(x) = subscribe.pop() {
                        println!("Подписываемся на топик {}", &x);
                        let _ = self.client.subscribe(x, QoS::AtLeastOnce).await;
                    }
                }
                Ok(Event::Incoming(Incoming::SubAck(s))) => {
                    println!("✅ MQTT: SubAck подтверждение от брокера {:?}", s);
                    if let Some(x) = subscribe.pop() {
                        println!("Подписываемся на топик {}", &x);
                        let _ = self.client.subscribe(x, QoS::AtLeastOnce).await;
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    eprintln!("⚠️ MQTT ошибка: {}", e);
                    // при ошибке — пауза, чтобы не спамить
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
