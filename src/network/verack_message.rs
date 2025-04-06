#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VerAckMessage {
    pub command: Vec<u8>
}
impl VerAckMessage {
    pub fn new() -> Self {
        VerAckMessage {
            command: b"verack".to_vec()
        }
    }
    pub fn parse() -> Self {
        VerAckMessage::new()
    }
    pub fn serialize(&self) -> Vec<u8> {
        Vec::new()
    }
}