use std::io::{Cursor};
use std::net::TcpStream;
use crate::block::Block;
use crate::helpers::varint::{read_varint, read_varint_tcp};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct HeadersMessage {
    pub command: Vec<u8>,
    pub blocks: Vec<Block>,
}
impl HeadersMessage {
    pub fn new(blocks: Vec<Block>) -> Self {
        HeadersMessage {
            command: b"headers".to_vec(),
            blocks
        }
    }
    pub fn parse_tcp(stream: &mut TcpStream, _testnet: bool) -> Result<Self, std::io::Error> {
        let num_header = read_varint_tcp(stream)?;
        let mut blocks: Vec<Block> = vec![];
        for _ in 0..num_header {
            blocks.push(Block::parse_tcp(stream).unwrap());
            let num_tx = read_varint_tcp(stream)?;
            if num_tx != 0 {
                panic!("Invalid tx number: {}", num_tx);
            }
        }
        Ok(HeadersMessage::new(blocks))
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>, _testnet: bool) -> Result<Self, std::io::Error> {
        let num_header = read_varint(stream)?;
        let mut blocks: Vec<Block> = vec![];
        for _ in 0..num_header {
            blocks.push(Block::parse(stream).unwrap());
            let num_tx = read_varint(stream)?;
            if num_tx != 0 {
                panic!("Invalid tx number: {}", num_tx);
            }
        }
        Ok(HeadersMessage::new(blocks))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_headers_message() {
        let msg_hex = hex::decode("0200000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001ac157e670000000002030eb2540c41025690160a1014c577061596e32e426b712c7ca00000000000000768b89f07044e6130ead292a3f51951adbd2202df447d98789339937fd006bd44880835b67d8001ade09204600").unwrap();
        let mut cursor = Cursor::new(msg_hex.clone());
        let headers_message = HeadersMessage::parse(&mut cursor, true).unwrap();
        assert_eq!(headers_message.blocks.len(), 2);
    }
}