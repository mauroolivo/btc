use std::io::{Cursor, Read};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PingMessage {
    command: Vec<u8>,
    nonce: Vec<u8>,
}
impl PingMessage {
    pub fn new(nonce: [u8;8]) -> Self {
        PingMessage {
            command: b"ping".to_vec(),
            nonce: nonce.to_vec(),
        }
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, std::io::Error> {
        let mut buffer = [0; 8];
        stream.read(&mut buffer)?;
        Ok(PingMessage::new(buffer))
    }
    pub fn serialize(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}