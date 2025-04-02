use std::fmt;
use std::fmt::Error;
use std::io::{Cursor, ErrorKind, Read};

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
    pub fn parse(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Result<Self, Error> {
        // let mut magic = [0; 4];
        // stream.read(&mut magic)?;
        // if magic == b"".as_slice() {
        //     return Err(Error::new(
        //         ErrorKind::InvalidData,
        //         "Connection reset!",
        //     ));
        // }

        Result::Err(Default::default())
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
    use crate::network::envelope::NETWORK_MAGIC;
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
}