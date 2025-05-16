use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

pub struct SimpleNode {
    pub testnet: bool,
    pub ipv4addr: Ipv4Addr,
    pub port: u16,
    pub tcp_stream: TcpStream
}
impl SimpleNode {
    pub fn new(host_ip: String, port: Option<u16>, testnet: bool) -> Self {
        let mut host = vec![];
        for item in host_ip.split(".").collect::<Vec<&str>>() {
            host.push(item.parse::<u8>().unwrap())
        }
        let port = if port.is_some() { port.unwrap() } else {
            if testnet { 18333u16 } else { 8333u16 }
        };
        let ipv4addr = Ipv4Addr::new(host[0], host[1], host[2], host[3]);
        let tcp_stream = TcpStream::connect(SocketAddrV4::new(ipv4addr, port)).expect("Failed to connect to server");
        SimpleNode { testnet, ipv4addr, port, tcp_stream }
    }
}
#[cfg(test)]
mod tests {
    use crate::network::simple_node::SimpleNode;
    use std::io::{Cursor, Write};
    use std::net::{Shutdown};
    use num::BigUint;
    use crate::network::envelope::NetworkEnvelope;
    use crate::network::get_headers_message::{GetHeadersMessage, GENESIS_BLOCK};
    use crate::network::headers_message::HeadersMessage;
    use crate::network::pong_message::PongMessage;
    use crate::network::verack_message::VerAckMessage;
    use crate::network::version_message::VersionMessage;

    #[test]
    fn test_socket_flow() {

        // Simple flow to get first 2000 header from generis block
        dotenv::dotenv().ok();
        let host_ip: String = std::env::var("HOST_IP").expect("Missing .env file or value");
        let mut node = SimpleNode::new(host_ip, Some(8333u16), false);

        println!("Successfully connected to server on port {}", node.port);

        // SEND VERSION
        let nonce: &[u8; 8] = b"\x00\x00\x00\x00\x00\x00\x00\x00";
        let message = VersionMessage::new(Some(BigUint::from(0u32)), *nonce);
        let envelope = NetworkEnvelope::new(message.command.clone(), message.serialize(), node.testnet);
        let msg = envelope.serialize();
        node.tcp_stream.write(msg.as_slice()).expect("Failed to write to stream");
        println!("Sent: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // RECEIVE
        let envelope = NetworkEnvelope::parse_tcp(&mut node.tcp_stream, node.testnet).expect("Failed to parse tcp message");
        println!("Received: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // SEND VERACK
        let message = VerAckMessage::new();
        let envelope = NetworkEnvelope::new(message.command.clone(), message.serialize(), false);
        let msg = envelope.serialize();
        node.tcp_stream.write(msg.as_slice()).expect("Failed to write to stream");
        println!("Sent: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // RECEIVE
        let envelope = NetworkEnvelope::parse_tcp(&mut node.tcp_stream, node.testnet).expect("Failed to parse tcp message");
        println!("Received: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // SEND GETHEADERS
        let prev_block_raw = hex::decode(GENESIS_BLOCK).unwrap();
        let message: GetHeadersMessage = GetHeadersMessage::new(prev_block_raw, None);
        let envelope = NetworkEnvelope::new(message.command.clone(), message.serialize(), node.testnet);
        let msg = envelope.serialize();
        node.tcp_stream.write(msg.as_slice()).expect("Failed to write to stream");
        println!("Sent: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // RECEIVE
        let envelope = NetworkEnvelope::parse_tcp(&mut node.tcp_stream, node.testnet).expect("Failed to parse tcp message");
        println!("Received: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // RECEIVE
        let envelope = NetworkEnvelope::parse_tcp(&mut node.tcp_stream, node.testnet).expect("Failed to parse tcp message");
        println!("Received: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // SEND PONG
        let nonce: &[u8; 8] = b"\x00\x00\x00\x00\x00\x00\x00\x00";
        let message: PongMessage = PongMessage::new(*nonce);
        let envelope = NetworkEnvelope::new(message.command.clone(), message.serialize(), node.testnet);
        let msg = envelope.serialize();
        node.tcp_stream.write(msg.as_slice()).expect("Failed to write to stream");
        println!("Sent: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // RECEIVE
        let envelope = NetworkEnvelope::parse_tcp(&mut node.tcp_stream, node.testnet).expect("Failed to parse tcp message");
        println!("Received: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()));

        // RECEIVE
        let envelope = NetworkEnvelope::parse_tcp(&mut node.tcp_stream, node.testnet).expect("Failed to parse tcp message");
        println!("Received: {:?} {:?}", String::from_utf8(envelope.command.clone()).unwrap(), hex::encode(envelope.serialize()).len());

        println!("Headers payload length: {:?}", envelope.payload.len());
        let pl = envelope.payload.clone();
        let mut cursor = Cursor::new(pl);
        let hm = HeadersMessage::parse(&mut cursor, false).expect("Failed to parse headers message");

        println!("Got headers, num of blocks: {:?}", hm.blocks.len());

        match node.tcp_stream.shutdown(Shutdown::Both) {
            Ok(_) => {
                println!("Socket shut down...");
            },
            Err(e) => {
                println!("Failed to shut down: {}", e);
            }
        }
    }
}