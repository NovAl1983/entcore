use std::time::Duration;

use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use cfg::MqttBroker;

pub  struct  MqttBridge {
    client: AsyncClient,
    eventloop: rumqttc::EventLoop,
}

impl MqttBridge {

    pub fn new(config: &MqttBroker) -> Self {
        let mut options = MqttOptions::new("entcore", &config.host, config.port);
        options.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(options, 50);

        Self { client, eventloop }
    }

    pub async fn subscribe(&self, topic: &str) -> Result<(), rumqttc::ClientError> {
        self.client.subscribe(topic, QoS::AtLeastOnce).await
    }

    pub async fn run(mut self) {
        loop {
            match self.eventloop.poll().await {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    let payload = String::from_utf8_lossy(&p.payload);
                    println!("📩 MQTT: topic='{}', payload='{}'", p.topic, payload);
                }
                Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                    println!("✅ MQTT: подключено к брокеру");
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