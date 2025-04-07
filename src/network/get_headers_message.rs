use num::BigUint;
use crate::helpers::endianness::int_to_little_endian;
use crate::helpers::varint::encode_varint;

pub const GENESIS_BLOCK: &str = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4a29ab5f49ffff001d1dac2b7c";
pub const TESTNET_GENESIS_BLOCK: &str = "0100000000000000000000000000000000000000000000000000000000000000000000003ba3edfd7a7b12b27ac72c3e67768f617fc81bc3888a51323a9fb8aa4b1e5e4adae5494dffff001d1aa4ae18";
pub const LOWEST_BITS: &str = "ffff001d";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct GetHeadersMessage {
    pub command: Vec<u8>,
    version: BigUint,
    num_hashes: u64,
    start_block: Vec<u8>,
    end_block: Vec<u8>
}
impl GetHeadersMessage {
    pub fn new(start_block: Vec<u8>, end_block: Option<Vec<u8>>) -> Self {
        let end_block: Vec<u8> = end_block.unwrap_or_else(|| vec![0u8; 32]);
        GetHeadersMessage {
            command: b"getheaders".to_vec(),
            version: BigUint::from(70015u32),
            num_hashes: 1u64,
            start_block,
            end_block
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(int_to_little_endian(self.version.clone(), 4));
        result.extend(encode_varint(self.num_hashes).unwrap());
        let mut rev_start_block = self.start_block.clone();
        rev_start_block.reverse();
        result.extend(rev_start_block);
        let mut rev_end_block = self.end_block.clone();
        rev_end_block.reverse();
        result.extend(rev_end_block);
        result
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_headers_message() {
        let block_hex = hex::decode("0000000000000000001237f46acddf58578a37e213d2a6edc4884a2fcad05ba3").unwrap();
        let gh = GetHeadersMessage::new(block_hex.clone(), None);
        assert_eq!(gh.serialize(), hex::decode("7f11010001a35bd0ca2f4a88c4eda6d213e2378a5758dfcd6af437120000000000000000000000000000000000000000000000000000000000000000000000000000000000").unwrap());
    }
}