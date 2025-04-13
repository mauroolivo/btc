use std::io::{Cursor, Read};
use crate::helpers::endianness::little_endian_to_int;
use num::{ToPrimitive};
use crate::helpers::merkle_hash::bytes_to_bit_field;
use crate::helpers::varint::read_varint;
use crate::merkle_tree::MerkleTree;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MerkleBlock {
    version: u32,
    prev_block: Vec<u8>,
    merkle_root: Vec<u8>,
    timestamp: u32,
    bits: Vec<u8>,
    nonce: Vec<u8>,
    total: u32,
    hashes: Vec<Vec<u8>>,
    flags: Vec<u8>,
}
impl MerkleBlock {
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
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let total = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();
        let mut hashes: Vec<Vec<u8>> = vec![];
        if let Ok(number_of_tx_hashes) = read_varint(stream) {
            for _ in 0..number_of_tx_hashes {
                let mut buffer = [0; 32];
                stream.read(&mut buffer)?;
                let mut hash = buffer.to_vec();
                hash.reverse();
                hashes.push(hash);
            }
        }
        let mut flags: Vec<u8> = vec![];
        if let Ok(lenght_flags_field) = read_varint(stream) {
            let mut buffer: Vec<u8> = vec![0u8; lenght_flags_field as usize];
            stream.read(&mut buffer)?;
            flags = buffer.to_vec();
        }
        Ok(MerkleBlock { version, prev_block, merkle_root, timestamp, bits, nonce, total, hashes, flags })
    }
    pub fn is_valid(&self) -> bool {
        let flag_bits = bytes_to_bit_field(self.flags.clone());
        let hashes: Vec<Vec<u8>> = self.hashes.clone().into_iter().map(|x| {
            let mut hash = x;
            hash.reverse();
            return hash;
        }).collect();
        let mut mt = MerkleTree::new(self.total.clone() as usize);
        mt.populate_tree(flag_bits, hashes);
        let mut root_reversed = mt.root();
        root_reversed.reverse();
        self.merkle_root == root_reversed
    }
}
#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use num::ToPrimitive;
    use crate::helpers::endianness::little_endian_to_int;
    use crate::merkle_block::MerkleBlock;

    #[test]
    fn test_merkle_block_parse() {
        let hex_merkle_block = hex::decode("00000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001ac157e670bf0d00000aba412a0d1480e370173072c9562becffe87aa661c1e4a6dbc305d38ec5dc088a7cf92e6458aca7b32edae818f9c2c98c37e06bf72ae0ce80649a38655ee1e27d34d9421d940b16732f24b94023e9d572a7f9ab8023434a4feb532d2adfc8c2c2158785d1bd04eb99df2e86c54bc13e139862897217400def5d72c280222c4cbaee7261831e1550dbb8fa82853e9fe506fc5fda3f7b919d8fe74b6282f92763cef8e625f977af7c8619c32a369b832bc2d051ecd9c73c51e76370ceabd4f25097c256597fa898d404ed53425de608ac6bfe426f6e2bb457f1c554866eb69dcb8d6bf6f880e9a59b3cd053e6c7060eeacaacf4dac6697dac20e4bd3f38a2ea2543d1ab7953e3430790a9f81e1c67f5b58c825acf46bd02848384eebe9af917274cdfbb1a28a5d58a23a17977def0de10d644258d9c54f886d47d293a411cb6226103b55635").unwrap();
        let mut stream = Cursor::new(hex_merkle_block);
        let mb = MerkleBlock::parse(&mut stream).unwrap();
        let version = 0x20000000;
        assert_eq!(mb.version, version);
        let mut merkle_root_hex = hex::decode("ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4").unwrap();
        merkle_root_hex.reverse();
        assert_eq!(mb.merkle_root, merkle_root_hex);
        let mut prev_block_hex = hex::decode("df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000").unwrap();
        prev_block_hex.reverse();
        assert_eq!(mb.prev_block, prev_block_hex);
        let timestamp_hex = hex::decode("dc7c835b").unwrap();
        let timestamp = little_endian_to_int(timestamp_hex.as_slice()).to_u32().unwrap();
        assert_eq!(mb.timestamp, timestamp);
        let bits_hex = hex::decode("67d8001a").unwrap();
        assert_eq!(mb.bits, bits_hex);
        let nonce_hex = hex::decode("c157e670").unwrap();
        assert_eq!(mb.nonce, nonce_hex);
        let total_hex = hex::decode("bf0d0000").unwrap();
        let total = little_endian_to_int(total_hex.as_slice()).to_u32().unwrap();
        assert_eq!(mb.total, total);
        let hex_hashes = [
            "ba412a0d1480e370173072c9562becffe87aa661c1e4a6dbc305d38ec5dc088a",
            "7cf92e6458aca7b32edae818f9c2c98c37e06bf72ae0ce80649a38655ee1e27d",
            "34d9421d940b16732f24b94023e9d572a7f9ab8023434a4feb532d2adfc8c2c2",
            "158785d1bd04eb99df2e86c54bc13e139862897217400def5d72c280222c4cba",
            "ee7261831e1550dbb8fa82853e9fe506fc5fda3f7b919d8fe74b6282f92763ce",
            "f8e625f977af7c8619c32a369b832bc2d051ecd9c73c51e76370ceabd4f25097",
            "c256597fa898d404ed53425de608ac6bfe426f6e2bb457f1c554866eb69dcb8d",
            "6bf6f880e9a59b3cd053e6c7060eeacaacf4dac6697dac20e4bd3f38a2ea2543",
            "d1ab7953e3430790a9f81e1c67f5b58c825acf46bd02848384eebe9af917274c",
            "dfbb1a28a5d58a23a17977def0de10d644258d9c54f886d47d293a411cb62261",
        ];
        let hashes: Vec<Vec<u8>> = hex_hashes.into_iter().map(|x| {
            let mut hash = hex::decode(x).unwrap();
            hash.reverse();
            return hash;
        }).collect();
        assert_eq!(mb.hashes, hashes);
        let flags = hex::decode("b55635").unwrap();
        assert_eq!(mb.flags, flags);
    }
    #[test]
    fn test_is_valid_merkle_block() {
        let hex_merkle_block = hex::decode("00000020df3b053dc46f162a9b00c7f0d5124e2676d47bbe7c5d0793a500000000000000ef445fef2ed495c275892206ca533e7411907971013ab83e3b47bd0d692d14d4dc7c835b67d8001ac157e670bf0d00000aba412a0d1480e370173072c9562becffe87aa661c1e4a6dbc305d38ec5dc088a7cf92e6458aca7b32edae818f9c2c98c37e06bf72ae0ce80649a38655ee1e27d34d9421d940b16732f24b94023e9d572a7f9ab8023434a4feb532d2adfc8c2c2158785d1bd04eb99df2e86c54bc13e139862897217400def5d72c280222c4cbaee7261831e1550dbb8fa82853e9fe506fc5fda3f7b919d8fe74b6282f92763cef8e625f977af7c8619c32a369b832bc2d051ecd9c73c51e76370ceabd4f25097c256597fa898d404ed53425de608ac6bfe426f6e2bb457f1c554866eb69dcb8d6bf6f880e9a59b3cd053e6c7060eeacaacf4dac6697dac20e4bd3f38a2ea2543d1ab7953e3430790a9f81e1c67f5b58c825acf46bd02848384eebe9af917274cdfbb1a28a5d58a23a17977def0de10d644258d9c54f886d47d293a411cb6226103b55635").unwrap();
        let mb = MerkleBlock::parse(&mut Cursor::new(hex_merkle_block)).unwrap();
        assert!(mb.is_valid());
    }
}