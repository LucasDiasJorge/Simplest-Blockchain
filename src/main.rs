mod blockchain;
mod kafka_consumer;
mod block;

use futures::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use blockchain::Blockchain;
use kafka_consumer::handle_message;

#[tokio::main]
async fn main() {
    // Kafka Consumer Config
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "rust-kafka-consumer")
        .set("bootstrap.servers", "localhost:9092")
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["my-topic"]).expect("Can't subscribe to specified topics");

    println!("Waiting messages ...");

    let mut message_stream = consumer.stream();
    let mut blockchain = Blockchain::new();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(borrowed_message) => handle_message(borrowed_message, &mut blockchain).await,
            Err(e) => eprintln!("Kafka error: {}", e),
        }
        blockchain.print();
        println!("Is Blockchain valid ? {}", blockchain.is_valid());
    }
}
