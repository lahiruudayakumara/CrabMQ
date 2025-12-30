use anyhow::Result;
use crabmq::protocol::types::{Packet, Publish};

#[tokio::main]
async fn main() -> Result<()> {
    let packet = Packet::Publish(Publish { topic: "test/topic".into(), payload: b"hello".to_vec() });
    let encoded = crabmq::protocol::encoder::encode_packet(&packet);
    println!("encoded {} bytes", encoded.len());
    Ok(())
}
