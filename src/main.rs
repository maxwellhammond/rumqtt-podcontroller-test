mod mqtt;
use mqtt::{establishclient, publishmessage, sub};
use std::io::{self, Write};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let address = args[0].to_string();
    let port = 4832;
    let pod = args[1].to_string();

    println!("Connect to {} on {} with pod {}", address, port, pod);
    //create topic
    let topic = format!("podc/v1/devices/{}", pod);

    let client = establishclient(pod, address, port, topic.clone()).await;
     loop {
        let mut message = String::new();
        print!("Enter message to send: ");
        io::stdout().flush().unwrap(); // Ensure the prompt prints immediately
        io::stdin().read_line(&mut message).unwrap();
        //let mqtt_client = establishclient(pod.clone(), address.clone(), port, topic.clone());
        let input = message.clone();
        let pub_topic = topic.clone();

        publishmessage(input, client.clone(), pub_topic).await;
     }
}
