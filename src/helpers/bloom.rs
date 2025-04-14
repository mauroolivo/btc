#[cfg(test)]
mod tests {
    use num::{BigUint, ToPrimitive};
    use crate::helpers::bloom::tests::HashFunctions::{Hash160, Hash256};
    use crate::helpers::hash160::hash160;
    use crate::helpers::hash256::hash256;

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
}