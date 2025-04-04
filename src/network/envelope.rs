use std::fmt;
use std::io::{Cursor, ErrorKind, Read};
use std::io::ErrorKind::InvalidData;
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use crate::helpers::hash256::hash256;

pub const NETWORK_MAGIC: &[u8; 4] = b"\xf9\xbe\xb4\xd9";
pub const TESTNET_NETWORK_MAGIC: &[u8; 4] = b"\x0b\x11\x09\x07";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct NetworkEnvelope {
    command: Vec<u8>,
    payload: Vec<u8>,
    magic: Vec<u8>
}
impl NetworkEnvelope {
    pub fn new(command: Vec<u8>, payload: Vec<u8>, testnet: bool) -> Self {
        let magic = match testnet {
            true => TESTNET_NETWORK_MAGIC.to_vec(),
            false => NETWORK_MAGIC.to_vec()
        };
        NetworkEnvelope { command, payload, magic }
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Result<Self, std::io::Error> {
        let mut magic = [0; 4];
        stream.read(&mut magic)?;
        if magic == b"".as_slice() {
            return Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Connection reset!",
            ));
        }
        let expected_magig = match testnet {
            true => TESTNET_NETWORK_MAGIC,
            false => NETWORK_MAGIC
        };
        if magic.to_vec() != expected_magig.to_vec() {
            let res = format!("magic is not right {} vs {}", hex::encode(magic), hex::encode(expected_magig));
            return Err(std::io::Error::new(InvalidData, res));
        }
        let mut buffer: [u8;12] = [0; 12];
        stream.read(&mut buffer)?;
        let mut command = buffer
            .into_iter()
            .rev()
            .skip_while(|&byte| byte == 0)
            .collect::<Vec<_>>();
        command.reverse();
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let payload_length = little_endian_to_int(&buffer);

        let mut checksum = [0; 4];
        stream.read(&mut checksum)?;

        let mut payload: Vec<u8> = vec![0; payload_length.to_usize().unwrap()];
        stream.read(&mut payload)?;
        let hash = hash256(&payload);
        if checksum.as_slice() != (hash[0..4]).iter().as_slice() {
            return Err(std::io::Error::new(InvalidData, "Checksum mismatch!"));
        }
        Ok(NetworkEnvelope::new(command, payload, testnet))
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(&self.magic);
        let pad = vec![0;12 - self.command.len()];
        let mut command = Vec::new();
        command.extend(&self.command);
        command.extend(&pad);
        result.extend(&command);
        result.extend(int_to_little_endian(BigUint::from(self.payload.len()), 4));
        let hash = hash256(&self.payload);
        result.extend(hash[0..4].to_vec());
        result.extend(&self.payload);
        result
    }
}
impl fmt::Display for NetworkEnvelope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "magic: {}, command: {}, payload: {:?}",
            hex::encode(&self.magic),
            String::from_utf8(self.command.clone()).unwrap(),
            self.payload.clone()
        )
    }
}
#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};
    use crate::helpers::endianness::little_endian_to_int;
    use crate::network::envelope::{NetworkEnvelope, NETWORK_MAGIC};
    #[test]
    fn test_network_message() {
        let raw_message = hex::decode("f9beb4d976657261636b000000000000650000005df6e0e2").unwrap(); // "verack\0\0\0\0\0\0"
        //let raw_message = hex::decode("f9beb4d976657273696f6e0000000000000000005df6e0e2").unwrap(); // "version\0\0\0\0\0"

        let mut stream = Cursor::new(raw_message);
        let mut buffer = [0; 4];
        _ = stream.read(&mut buffer);
        let magic = buffer.to_vec();
        let mut buffer = [0; 12];
        _ = stream.read(&mut buffer);
        let command = buffer.to_vec();
        let mut buffer = [0; 4];
        _ = stream.read(&mut buffer);
        let payload_len = little_endian_to_int(&buffer);
        assert_eq!(magic, *NETWORK_MAGIC);
        println!("{:?}", hex::encode(&magic));
        println!("{:?}", String::from_utf8(command).unwrap());
        println!("{:?}", payload_len);
    }
    #[test]
    fn test_network_envelope_parse() {

        let raw_message = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
        let mut stream = Cursor::new(raw_message);
        let envelope = NetworkEnvelope::parse(&mut stream, false).unwrap();

        assert_eq!(String::from_utf8(envelope.command).unwrap(), String::from("verack"));
        assert_eq!(envelope.payload, b"");

        let raw_message = hex::decode("f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001").unwrap();
        let mut stream = Cursor::new(raw_message.clone());
        let envelope = NetworkEnvelope::parse(&mut stream, false).unwrap();
        assert_eq!(envelope.command, b"version");
        let want = &raw_message[24..raw_message.len()];
        assert_eq!(envelope.payload, want);
    }
    #[test]
    fn test_network_envelope_serialize() {
        let raw_message = hex::decode("f9beb4d976657261636b000000000000000000005df6e0e2").unwrap();
        let mut stream = Cursor::new(raw_message.clone());
        let envelope = NetworkEnvelope::parse(&mut stream, false).unwrap();
        assert_eq!(envelope.serialize(), raw_message);

        let raw_message = hex::decode("f9beb4d976657273696f6e0000000000650000005f1a69d2721101000100000000000000bc8f5e5400000000010000000000000000000000000000000000ffffc61b6409208d010000000000000000000000000000000000ffffcb0071c0208d128035cbc97953f80f2f5361746f7368693a302e392e332fcf05050001").unwrap();
        let mut stream = Cursor::new(raw_message.clone());
        let envelope = NetworkEnvelope::parse(&mut stream, false).unwrap();
        assert_eq!(envelope.serialize(), raw_message);
    }
}