use crabmq::protocol::{decoder, encoder, types::{Packet, Publish}};

#[test]
fn encode_decode_publish() {
    let pkt = Packet::Publish(Publish{ topic: "t".into(), payload: b"abc".to_vec()});
    let enc = encoder::encode_packet(&pkt).freeze();
    let dec = decoder::decode_packet(enc).expect("decode");
    match dec { Packet::Publish(p) => assert_eq!(p.payload, b"abc"), }
}
