#[cfg(test)]
mod tests {
    use num::{BigUint, ToPrimitive};
    use crate::helpers::hash256::hash256;

    #[test]
    fn test_bloom_1() {
        let bit_field_size = 10u32;
        let mut bit_filed: Vec<u8> = vec![0; bit_field_size as usize];
        let hash = hash256(b"hello world");
        let hash_be = BigUint::from_bytes_be(&hash);
        let bit = hash_be % bit_field_size;
        bit_filed[bit.to_usize().unwrap()] = 1;
        println!("{:?}", bit_filed);
    }
}