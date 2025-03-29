use chrono::{Utc, DateTime};
use std::{io::{Cursor, Read}};
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use crate::helpers::hash256::hash256;

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
    pub fn serialize(&self) -> Vec<u8> {
        // Returns the 80 byte block header
        let mut result = Vec::new();
        result.extend(int_to_little_endian(BigUint::from(self.version), 4));
        let mut prev_block = self.prev_block.clone();
        prev_block.reverse();
        result.extend(&prev_block);
        let mut merkle_root = self.merkle_root.clone();
        merkle_root.reverse();
        result.extend(&merkle_root);
        result.extend(int_to_little_endian(BigUint::from(self.timestamp), 4));
        result.extend(self.bits.clone());
        result.extend(self.nonce.clone());
        result
    }
    pub fn hash(&self) -> Vec<u8> {
        let bytes = self.serialize();
        let mut hash = hash256(&bytes);
        hash.reverse();
        hash.to_vec()
    }
    pub fn bip_readiness_check(&self, n: u32) -> Option<bool> {
        match n {
            9 => {
                Some(self.version >> 29 == 0b001)
            }
            91 => {
                let shift = self.version >> 4;
                Some(shift & 1 == 1)
            }
            141 => { //segwit
                let shift = self.version >> 1;
                Some(shift & 1 == 1)
            }
            341 => { //taproot
                let shift = self.version >> 2;
                println!("{:?}", shift.to_le_bytes());
                Some(shift & 1 == 1)
            }
            _ => None
        }
    }
    pub fn time_to_date(&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.timestamp as i64, 0).unwrap()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_block() {
        let block_raw = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut cursor = Cursor::new(block_raw);
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
    #[test]
    fn test_block_serialize() {
        let block_raw = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.serialize(), block_raw);
    }
    #[test]
    fn test_hash() {
        let block_raw = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        let hash = block.hash();
        let hash = hex::encode(hash);
        println!("{:?}", hash);
        assert_eq!(hash, "0000000000000000007e9e4c586439b0cdbe13b1370bdd9435d76a644d047523");
    }
    #[test]
    fn test_bip_9() {
        let block_raw = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.bip_readiness_check(9).unwrap(), true);
        let block_raw = hex::decode("0400000039fa821848781f027a2e6dfabbf6bda920d9ae61b63400030000000000000000ecae536a304042e3154be0e3e9a8220e5568c3433a9ab49ac4cbb74f8df8e8b0cc2acf569fb9061806652c27").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.bip_readiness_check(9).unwrap(), false);
    }
    #[test]
    fn test_bip_91() {
        let block_raw = hex::decode("1200002028856ec5bca29cf76980d368b0a163a0bb81fc192951270100000000000000003288f32a2831833c31a25401c52093eb545d28157e200a64b21b3ae8f21c507401877b5935470118144dbfd1").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.bip_readiness_check(91).unwrap(), true);
        let block_raw = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.bip_readiness_check(91).unwrap(), false);
    }
    #[test]
    fn test_bip_141() {
        let block_raw = hex::decode("020000208ec39428b17323fa0ddec8e887b4a7c53b8c0a0a220cfd0000000000000000005b0750fce0a889502d40508d39576821155e9c9e3f5c3157f961db38fd8b25be1e77a759e93c0118a4ffd71d").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.bip_readiness_check(141).unwrap(), true);
        let block_raw = hex::decode("0000002066f09203c1cf5ef1531f24ed21b1915ae9abeb691f0d2e0100000000000000003de0976428ce56125351bae62c5b8b8c79d8297c702ea05d60feabb4ed188b59c36fa759e93c0118b74b2618").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.bip_readiness_check(141).unwrap(), false);
    }
    #[test]
    fn test_bip_341() {
        let block_raw = hex::decode("04002020ccbcc674693ef8751c939c0e6d4728dde62e24fc12370100000000000000000077ec1447375fc68029ab7a85fd6989c5d31351b619e8f709de682008103bda6a6f9b9061ea690c1702730f54").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        assert_eq!(block.bip_readiness_check(341).unwrap(), true);
    }
    #[test]
    fn test_time_to_datetime() {
        let block_raw = hex::decode("04002020ccbcc674693ef8751c939c0e6d4728dde62e24fc12370100000000000000000077ec1447375fc68029ab7a85fd6989c5d31351b619e8f709de682008103bda6a6f9b9061ea690c1702730f54").unwrap();
        let mut cursor = Cursor::new(block_raw.clone());
        let block = Block::parse(&mut cursor).unwrap();
        println!("{}", block.time_to_date())
    }
}
