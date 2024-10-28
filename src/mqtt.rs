use rumqttc::{AsyncClient, ClientError, Event, EventLoop, Incoming, MqttOptions, QoS};
use tokio::time;
use std::sync::{Arc, Mutex};
use std::time::Duration;


pub async fn establishclient(pod: String, address: String, port: u16, topic: String) -> rumqttc::AsyncClient {
    //establish client with options
    let mut mqttoptions = MqttOptions::new(pod, address, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let messages = Arc::new(Mutex::new(Vec::new()));

    //subscribe to topic
    if let Err(e) = client.subscribe(topic, QoS::AtMostOnce).await {
        eprintln!("Failed to subscribe: {:?}", e)
    } else {
        println!("Subscribed");
    }
    let messages_clone = Arc::clone(&messages);
    
    //create async thread for the eventloop
    tokio::spawn(async move {
        while let Ok(notification) = eventloop.poll().await {
            match notification {
                Event::Incoming(Incoming::Publish(publish)) => {
                    // Store received message in shared state
                    let payload = String::from_utf8_lossy(&publish.payload).to_string();
                    messages_clone.lock().unwrap().push(payload);
                }
                _ => {}
            }
        }
    });
    //client.subscribe(topic, QoS::AtMostOnce).await.unwrap();
    
    //return AsyncClient (Client details for publishing messages)
    client
}

//sub is not used for this program because it is handled when connection is made.
pub async fn sub (client: AsyncClient, topic: String) {
    //subscribe to topic
    client.subscribe(topic, QoS::AtMostOnce).await.unwrap();
    time::sleep(Duration::from_secs(1)).await;
}

pub async fn publishmessage (message: String, client: AsyncClient, topic: String) {
    //let message_bytes = message.as_bytes();
    let payload = message.trim();
    client.publish(topic, QoS::ExactlyOnce, false, payload).await.unwrap();
    time::sleep(Duration::from_millis(10)).await;
}