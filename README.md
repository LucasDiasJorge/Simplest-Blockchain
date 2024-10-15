# Simple Blockchain in Rust

This project showcases a basic blockchain implementation using Rust, expanded with Kafka integration for handling real-time data streams. Each Kafka message is treated as a new block in the blockchain, maintaining the principles of immutability and integrity.

## Features

- Basic **block structure** with an index, timestamp, data, hash, and reference to the previous block.
- **Genesis Block** initialization: The first block in the chain.
- Simple hash calculation to simulate block verification.
- Functions to **add new blocks** to the chain.
- A method to **validate** the blockchain by checking the linkage and integrity of the blocks.
- Kafka **Consumer Integration**: Listens to messages from a Kafka topic and adds them as blocks to the blockchain.

## New Kafka-Related Features

- **Kafka Consumer**: Consumes messages from a Kafka topic and adds them to the blockchain as new blocks.
- **Real-time Processing**: Blocks are added dynamically to the blockchain as messages arrive from Kafka.
- **Docker Integration**: Kafka is set up using Docker Compose for easy deployment.

## Concepts

This project demonstrates the following concepts:
- **Blockchain**: A chain of blocks where each block contains a reference (hash) to the previous block.
- **Hashing**: Simplified hash function based on the block's content (index, timestamp, previous hash, and data).
- **Kafka**: Used as a message broker, feeding data to the blockchain in real-time.

## Code Overview

- **`Block`**: Represents a block in the chain, holding the index, timestamp, data, and a reference to the previous block's hash.
- **`Blockchain`**: Manages the list of blocks, including methods for adding new blocks and validating the chain.
- **`Kafka Consumer`**: Subscribes to a topic and processes incoming messages, which are then added to the blockchain as new blocks.
- **`Genesis Block`**: The first block in the blockchain, initialized with static values since it has no previous block.

## Example Usage

Here’s a simple example showing how to run the blockchain with Kafka:

```rust
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

    let mut blockchain = Blockchain::new();

    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        match message {
            Ok(borrowed_message) => handle_message(borrowed_message, &mut blockchain).await,
            Err(e) => eprintln!("Kafka error: {}", e),
        }
        blockchain.print();
        println!("Is Blockchain valid? {}", blockchain.is_valid());
    }
}
```

### Docker Compose Setup

Make sure Kafka is running using the provided `docker-compose.yml` file. Here’s how to start Kafka:

```bash
docker-compose up -d
```

## Future Improvements

- Implement a **cryptographic hash** function (e.g., SHA-256) for enhanced security.
- Add **proof-of-work** for mining blocks to simulate more realistic blockchain behavior. (Done)
- Extend the project to handle **distributed consensus** and multiple nodes.

## Dependencies

- **Rust**
- **Kafka** (using the [librdkafka](https://github.com/edenhill/librdkafka) library in Rust)
- **Docker** (for Kafka setup)

## Running the Project

1. Clone the repository:

```bash
git clone https://github.com/LucasDiasJorge/Simplest-Blockchain
cd Simple-Blockchain
```

2. Run the Docker container for Kafka:

```bash
docker-compose up -d
```

3. Start the Rust application:

```bash
cargo run
```

4. Send messages to the Kafka topic, and observe how the blockchain grows in real-time.

## Resources

For more on how Kafka works, check out [Kafka Documentation](https://kafka.apache.org/documentation/).

For details on blockchain fundamentals, see the [AWS Blockchain Overview](https://aws.amazon.com/what-is/blockchain/).
