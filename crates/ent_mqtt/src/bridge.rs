

use std::println;
use std::time::Duration;

// use cfg::Mqtt;
use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use ent_core::{EntityHandle, UpdateSource};

use cfg::{MqttBroker, Mqtt};
// use crate::converter::{Topics, mqtt_from_config};





use crate::converter::{TopicKind, Topics, mqtt_from_config};

use ent_core::FieldUpdate;

// use ent_core::UpdateSource;
// use ent_core::EntityHandle;




pub  struct  MqttBridge {
    client: AsyncClient,
    eventloop: rumqttc::EventLoop,
    topics: Topics,
    // tx: EntityHandle,
    handle: EntityHandle
}

impl MqttBridge {

    pub fn new(broker: &MqttBroker, mqtt: &Mqtt, handle: EntityHandle) -> Self {
        let mut options = MqttOptions::new("entcore", &broker.host, broker.port);
        options.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(options, 50);

        let topics = mqtt_from_config(mqtt);


        Self { client, eventloop, topics, handle }
    }

    pub async fn run(mut self) {
        let mut subscribe: Vec<String> = Vec::new();
        loop {
            match self.eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    let payload = String::from_utf8_lossy(&p.payload);
                    if let Some(mapping) = self.topics.state_topic.get(&p.topic) {
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
        self.handle.update_field(mapping.entity_id.clone(), field, UpdateSource::Mqtt);
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
                    println!("✅ MQTT: SubAck подтверждение от брокера {:?}", s );
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

