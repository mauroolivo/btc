#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GenericMessage {
    command: Vec<u8>,
    payload: Vec<u8>,
}
impl GenericMessage {
    pub fn new(command: Vec<u8>, payload: Vec<u8> ) -> Self {
        GenericMessage {
            command,
            payload,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        self.payload.clone()
    }
}
