// ent_mqtt/examples/publisher.rs
use std::time::Duration;
use rumqttc::{AsyncClient, MqttOptions, QoS};

#[tokio::main]
async fn main() {
    let mut options = MqttOptions::new("test-pub", "localhost", 1883);
    options.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    
    // Крутим eventloop в фоне
    tokio::spawn(async move {
        loop {
            let _ = eventloop.poll().await;
        }
    });
    
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    client.publish(
        "/devices/wb-msw-v4_220/controls/Current Motion",
        QoS::AtLeastOnce,
        true,
        "1",
    ).await.unwrap();
    
    println!("📤 Опубликовано");
    tokio::time::sleep(Duration::from_millis(500)).await;
}