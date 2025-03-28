use std::{io::{Cursor, Read}};
use num::{ToPrimitive};
use crate::helpers::endianness::{little_endian_to_int};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    version: u32,
    prev_block: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: u32,
    bits: Vec<u8>,
    nonce: Vec<u8>
}
impl Block {
    pub fn new(version: u32, prev_block: Vec<u8>, merkle_root: Vec<u8>, timestamp: u32, bits: Vec<u8>, nonce: Vec<u8>) -> Self {
        Block {
            version, prev_block, merkle_root, timestamp, bits, nonce
        }
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, std::io::Error> {
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let version = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();
        let mut buffer = [0; 32];
        stream.read(&mut buffer)?;
        let mut prev_block = buffer.to_vec();
        prev_block.reverse();
        let mut buffer = [0; 32];
        stream.read(&mut buffer)?;
        let mut merkle_root = buffer.to_vec();
        merkle_root.reverse();
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let timestamp = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();

        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let bits = buffer.to_vec();
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let nonce = buffer.to_vec();

        Ok(Block::new(version, prev_block, merkle_root, timestamp, bits, nonce))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_block() {
        let block_row = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut cursor = Cursor::new(block_row);
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.version, 0x20000002);
        let want = hex::decode("000000000000000000fd0c220a0a8c3bc5a7b487e8c8de0dfa2373b12894c38e").unwrap();
        assert_eq!(block.prev_block, want);
        let want = hex::decode("be258bfd38db61f957315c3f9e9c5e15216857398d50402d5089a8e0fc50075b").unwrap();
        assert_eq!(block.merkle_root, want);
        assert_eq!(block.timestamp, 0x59a7771e);
        let want = hex::decode("e93c0118").unwrap();
        assert_eq!(block.bits, want);
        let want = hex::decode("a4ffd71d").unwrap();
        assert_eq!(block.nonce, want);
    }
}