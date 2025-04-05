use num::{BigUint, ToPrimitive};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::helpers::endianness::{int_to_little_endian};
use crate::helpers::varint::encode_varint;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct VersionMessage {
    command: Vec<u8>,
    version: BigUint,
    services: BigUint,
    timestamp: BigUint,
    receiver_services: BigUint,
    receiver_ip: Vec<u8>,
    receiver_port: BigUint,
    sender_services: BigUint,
    sender_ip: Vec<u8>,
    sender_port: BigUint,
    nonce: Vec<u8>,
    user_agent: Vec<u8>,
    latest_block: BigUint,
    relay: bool
}
impl VersionMessage {
    pub fn new(timestamp: Option<BigUint>, nonce: [u8;8]) -> Self {
        let t_stamp = timestamp.unwrap_or_else(|| BigUint::from(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
        VersionMessage {
            command: b"version".to_vec(),
            version: BigUint::from(70015u32),
            services: BigUint::from(0u32),
            timestamp: t_stamp,
            receiver_services: BigUint::from(0u32),
            receiver_ip: b"\x00\x00\x00\x00".to_vec(),
            receiver_port: BigUint::from(8333u32),
            sender_services: BigUint::from(0u32),
            sender_ip: b"\x00\x00\x00\x00".to_vec(),
            sender_port: BigUint::from(8333u32),
            nonce: nonce.to_vec(),
            user_agent: b"/programmingbitcoin:0.1/".to_vec(),
            latest_block: BigUint::from(0u32),
            relay: false
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(int_to_little_endian(self.version.clone(), 4));
        result.extend(int_to_little_endian(self.services.clone(), 8));
        result.extend(int_to_little_endian(self.timestamp.clone(), 8));

        result.extend(int_to_little_endian(self.receiver_services.clone(), 8));
        // IPV4 is 10 00 bytes and 2 ff bytes then receiver ip
        let pad = vec![0;10];
        result.extend(pad);
        result.extend(b"\xff\xff".to_vec());
        result.extend(&self.receiver_ip);
        // port is 2 bytes, big endian
        result.extend(&self.receiver_port.to_u16().unwrap().to_be_bytes());

        result.extend(int_to_little_endian(self.sender_services.clone(), 8));
        // IPV4 is 10 00 bytes and 2 ff bytes then receiver ip
        let pad = vec![0;10];
        result.extend(pad);
        result.extend(b"\xff\xff".to_vec());
        result.extend(&self.sender_ip);
        // port is 2 bytes, big endian
        result.extend(&self.sender_port.to_u16().unwrap().to_be_bytes());
        result.extend(&self.nonce);
        result.extend(encode_varint(self.user_agent.len() as u64).unwrap());
        result.extend(&self.user_agent);
        result.extend(int_to_little_endian(self.latest_block.clone(), 4));
        match self.relay {
            true => {
                result.extend(b"\x01");
            }
            false => {
                result.extend(b"\x00");
            }
        }
        result
    }
}
#[cfg(test)]
mod tests {
    use num::BigUint;
    use crate::network::version_message::VersionMessage;
    #[test]
    fn test_version_message_serialize() {

        let nonce: &[u8; 8] = b"\x00\x00\x00\x00\x00\x00\x00\x00";
        let message = VersionMessage::new(Some(BigUint::from(0u32)), *nonce);
        println!("{:?}", hex::encode(message.serialize()));
        assert_eq!(hex::encode(message.serialize()), "7f11010000000000000000000000000000000000000000000000000000000000000000000000ffff00000000208d000000000000000000000000000000000000ffff00000000208d0000000000000000182f70726f6772616d6d696e67626974636f696e3a302e312f0000000000");
    }
}