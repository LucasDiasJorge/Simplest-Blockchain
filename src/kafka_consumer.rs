use rdkafka::Message;
use rdkafka::message::BorrowedMessage;
use crate::blockchain::Blockchain;

pub async fn handle_message(msg: BorrowedMessage<'_>, blockchain: &mut Blockchain) {
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
