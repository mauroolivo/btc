static BIP37_CONSTANT: u32 = 0xfba4c795;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BloomFilter {
    size: u32,
    bit_field: Vec<u8>,
    function_count: u32,
    tweak: u32,
}
impl BloomFilter {
    pub fn new(size: u32, function_count: u32, tweak: u32) -> Self {
        let bit_field = vec![0; size as usize * 8];
        BloomFilter {size, bit_field, function_count, tweak }
    }
}
#[cfg(test)]
mod tests {
    use num::{BigUint, ToPrimitive};
    use crate::bloom_filter::{BloomFilter, BIP37_CONSTANT};
    use crate::bloom_filter::tests::HashFunctions::{Hash160, Hash256};
    use crate::helpers::hash160::hash160;
    use crate::helpers::hash256::hash256;
    use crate::helpers::merkle_hash::bit_field_to_bytes;

    enum HashFunctions {
        Hash256,
        Hash160,
    }
    #[test]
    fn test_bloom_1() {
        let bit_field_size = 10u32;
        let mut bit_field: Vec<u8> = vec![0; bit_field_size as usize];
        let hash = hash256(b"hello world");
        let hash_be = BigUint::from_bytes_be(&hash);
        let bit = hash_be % bit_field_size;
        bit_field[bit.to_usize().unwrap()] = 1;
        println!("{:?}", bit_field);
    }
    #[test]
    fn test_bloom_2() {
        let bit_field_size = 10u32;
        let mut bit_field: Vec<u8> = vec![0; bit_field_size as usize];
        let hashes = vec![hash256(b"hello world"), hash256(b"goodbye")];
        for hash in hashes {
            let hash_be = BigUint::from_bytes_be(&hash);
            let bit = hash_be % bit_field_size;
            bit_field[bit.to_usize().unwrap()] = 1;
        }
        println!("{:?}", bit_field);
    }
    #[test]
    fn test_bloom_3() {
        let bit_field_size = 10u32;
        let mut bit_field: Vec<u8> = vec![0; bit_field_size as usize];
        let phrases = vec!["hello world", "goodbye"];
        let hash_functions: Vec<HashFunctions> = vec![Hash256, Hash160];
        for phrase in phrases {
            for hash_function in hash_functions.iter() {
                match hash_function {
                    Hash256 => {
                        let hash = hash256(phrase.as_bytes());
                        let hash_be = BigUint::from_bytes_be(&hash);
                        let bit = hash_be % bit_field_size;
                        bit_field[bit.to_usize().unwrap()] = 1;
                    }
                    Hash160 => {
                        let hash = hash160(phrase.as_bytes());
                        let hash_be = BigUint::from_bytes_be(&hash);
                        let bit = hash_be % bit_field_size;
                        bit_field[bit.to_usize().unwrap()] = 1;
                    }
                }
            }
        }
        println!("{:?}", bit_field);
    }
    #[test]
    fn test_bloom_4() {
        let field_size = 2u32;
        let num_functions = 2u32;
        let tweak = 42u32;
        let bit_field_size = field_size * 8;
        let mut bit_field = vec![0; bit_field_size as usize];
        let phrases = vec!["hello world", "goodbye"];
        for phrase in phrases {
            for i in 0..num_functions {
                let seed = i * BIP37_CONSTANT + tweak;
                let h = murmur3::murmur3_32(&mut phrase.as_bytes(), seed).unwrap();
                let bit = h % bit_field_size;
                bit_field[bit.to_usize().unwrap()] = 1;
            }
        }
        println!("{:?}", bit_field);
    }
    #[test]
    fn test_bloom_5() {
        let mut bf = BloomFilter::new(10, 2, 42);
        let phrases = vec!["hello world", "goodbye"];
        for phrase in phrases {
            for i in 0..bf.function_count {
                let seed = i * BIP37_CONSTANT + bf.tweak;
                let h = murmur3::murmur3_32(&mut phrase.as_bytes(), seed).unwrap();
                let bit = h % bf.size;
                bf.bit_field[bit.to_usize().unwrap()] = 1;
            }
        }
        println!("{:?}", bf.bit_field);
    }
    #[test]
    fn test_bloom_6() {
        let mut bf = BloomFilter::new(10, 5, 99);
        let phrases = vec!["Hello World", "Goodbye!"];
        for phrase in phrases {
            for i in 0..bf.function_count {
                let seed = i as u128 * BIP37_CONSTANT as u128 + bf.tweak as u128;
                let h = murmur3::murmur3_32(&mut phrase.as_bytes(), seed as u32);
                let bit = h.unwrap() % (bf.size * 8) as u32;
                bf.bit_field[bit.to_usize().unwrap()] = 1;
            }
        }
        println!("{:?}", bf.bit_field);
        println!("{:?}", hex::encode(bit_field_to_bytes(bf.bit_field)));
    }
}