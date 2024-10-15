use std::time::{SystemTime, UNIX_EPOCH};
use futures::StreamExt;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::{ClientConfig, Message};

#[derive(Debug, Clone)]
struct Block {
    index: u64,
    timestamp: u128,
    previous_hash: u64,
    hash: u64,
    data: String,
}

impl Block {
    fn new(index: u64, previous_hash: u64, data: String) -> Block {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        let hash = Block::calculate_hash(index, timestamp, previous_hash, &data);

        Block {
            index,
            timestamp,
            previous_hash,
            hash,
            data,
        }
    }

    fn calculate_hash(index: u64, timestamp: u128, previous_hash: u64, data: &str) -> u64 {
        let mut hash = index + previous_hash;
        for byte in data.bytes() {
            hash += byte as u64;
        }
        hash += timestamp as u64;
        hash
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain {
            chain: vec![Blockchain::create_genesis_block()],
        }
    }

    // Origin block
    fn create_genesis_block() -> Block {
        Block::new(0, 0, String::from("Genesis Block"))
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap().clone();
        let new_block = Block::new(previous_block.index + 1, previous_block.hash, data);
        self.chain.push(new_block);
    }

    fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != Block::calculate_hash(
                current_block.index,
                current_block.timestamp,
                current_block.previous_hash,
                &current_block.data,
            ) {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        true
    }

    fn print(&self) {
        for block in &self.chain {
            println!("{:?}", block);
        }
    }
}

async fn handle_message(msg: BorrowedMessage<'_>, blockchain: &mut Blockchain) {
    let payload = match msg.payload_view::<str>() {
        Some(Ok(payload)) => payload,
        Some(Err(_)) => "<payload is not utf-8>",
        None => "<payload is empty>",
    };

    let topic = msg.topic();
    let partition = msg.partition();
    let offset = msg.offset();

    blockchain.add_block(payload.to_string());

    println!(
        "Received: topic: {}, partition: {}, offset: {}, payload: {}",
        topic, partition, offset, payload
    );
}

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