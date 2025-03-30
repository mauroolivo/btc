use num::{pow, BigUint};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};

pub fn bits_to_target(bits: &Vec<u8>) -> BigUint {
    // last byte is exponent
    let exponent = bits[bits.len() - 1];
    let coefficient = little_endian_to_int(&bits[0..bits.len() - 1]);
    coefficient * pow(BigUint::from(256u32), exponent as usize - 3)
}
pub fn target_to_bits(target: &BigUint) -> Vec<u8> {
    let raw_bytes = target.to_bytes_be();
    let mut new_bits: Vec<u8> = Vec::new();
    let mut coefficient: Vec<u8> = Vec::new();
    let exponent: usize;
    if raw_bytes[0] > 0x7f {
        exponent = raw_bytes.len() + 1;
        coefficient.extend(b"\x00");
        coefficient.extend(raw_bytes[0..raw_bytes.len() - 2].to_vec());
    } else {
        exponent = raw_bytes.len();
        coefficient.extend(raw_bytes[0..raw_bytes.len() - 3].to_vec());
    }
    coefficient.reverse();
    new_bits.extend(&coefficient);
    let exp_bytes = int_to_little_endian(BigUint::from(exponent), 1).to_vec();
    new_bits.extend(exp_bytes);
    remove_leading_zeros(new_bits)
}
pub fn remove_leading_zeros(bytes: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut pad = 0;
    for byte in bytes.clone() {
        if byte == 0x00 {
            pad += 1;
        }
    }
    result.extend(bytes[pad..].to_vec());
    result
}