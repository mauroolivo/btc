use std::io::{Cursor, Read};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PongMessage {
    pub command: Vec<u8>,
    nonce: Vec<u8>,
}
impl PongMessage {
    pub fn new(nonce: [u8;8]) -> Self {
        PongMessage {
            command: b"pong".to_vec(),
            nonce: nonce.to_vec(),
        }
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, std::io::Error> {
        let mut buffer = [0; 8];
        stream.read(&mut buffer)?;
        Ok(PongMessage::new(buffer))
    }
    pub fn serialize(&self) -> Vec<u8> {
        self.nonce.clone()
    }
}