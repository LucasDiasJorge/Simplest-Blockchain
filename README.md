# Simple Blockchain in Rust

This project demonstrates a basic implementation of a blockchain using Rust, inspired by the concept of a **Linked List**. The blockchain is composed of sequential blocks, where each block references the hash of the previous one, ensuring the integrity and immutability of the chain.

## Features

- Basic **block structure** with an index, timestamp, data, hash, and reference to the previous block.
- **Genesis Block** initialization: The first block in the chain.
- Simple hash calculation (without cryptographic functions) using basic operations to simulate block verification.
- Functions to **add new blocks** to the chain.
- A method to **validate** the blockchain by checking if the chain is correctly linked.

## Concepts

This project demonstrates the following concepts:
- **Blockchain**: A chain of blocks where each block contains a reference (hash) to the previous block.
- **Linked List**: The blockchain structure is similar to a singly linked list, where each block (node) points to its predecessor, ensuring a sequential and connected data flow.
- **Hashing**: Simplified hash function based on the block's content (index, timestamp, previous hash, and data) to ensure the chain's consistency.

## Code Overview

- **`Block`**: Represents a block in the chain, holding the index, timestamp, data, and a reference to the previous block's hash.
- **`Blockchain`**: Holds the vector of blocks and includes functions to add new blocks and validate the chain.
- **`Genesis Block`**: The first block in the blockchain, initialized with static values since it does not have a previous block.

## Example Usage

```rust
fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.add_block("Block 1 - MORE 10 BTC".to_string());
    blockchain.add_block("block 2 - LESS 5 BTC".to_string());

    blockchain.print();

    println!("Is Blockchain valid ? {}", blockchain.is_valid());
}
```

## Future Improvements

- Implement a **cryptographic hash** function such as SHA-256 for better security.
- Add **proof-of-work** for block mining to simulate more realistic blockchain behavior. (Done)
- Improve the validation process with more robust checks and error handling.

_[ref](https://aws.amazon.com/what-is/blockchain/)_
