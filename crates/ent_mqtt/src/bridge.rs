use std::{collections::HashMap, time::Duration};

use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, QoS};

use ent_core::{Entity, EntityState};

use cfg::{MqttBroker, Mqtt};



pub  struct  MqttBridge {
    client: AsyncClient,
    eventloop: rumqttc::EventLoop,
    // topics: Topics,
}

impl MqttBridge {

    pub fn new(cfg_broker: &MqttBroker, cfg_topic: &Mqtt) -> Self {
        let mut options = MqttOptions::new("entcore", &cfg_broker.host, cfg_broker.port);
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
                    println!("✅ MQTT: ConnAck подключено к брокеру");
                    let _ = self.client.subscribe("/devices/wb-msw-v4_220/controls/Current Motion", QoS::AtLeastOnce).await;
                }
                Ok(Event::Incoming(Incoming::SubAck(s))) => {
                    println!("✅ MQTT: SubAck подтвереждение от броекра {:?}", s );
                    // let _ = self.client.subscribe("/devices/wb-msw-v4_220/controls/Current Motion", QoS::AtLeastOnce).await;
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

