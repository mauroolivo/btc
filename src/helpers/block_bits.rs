use num::{pow, BigUint};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use num::Num;
use num::traits::Euclid;
pub const TWO_WEEKS: u32 = 60 * 60 * 24 * 14;

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
    last_4_bytes(new_bits)
}
pub fn last_4_bytes(bytes: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.extend(bytes[(bytes.len() - 4)..].to_vec());
    result
}
pub fn calculate_new_bits(previous_bits: Vec<u8>, time_diff: u32) -> Vec<u8> {
    //Calculates the new bits given a 2016-block time differential and the previous bits
    let mut time_differential = time_diff;

    // greater than 8 weeks, set to 8 weeks
    if time_differential > TWO_WEEKS * 4 {
        time_differential = TWO_WEEKS * 4;
    }
    // less than half a week, set to half a week
    if time_differential < TWO_WEEKS.div_euclid(4) {
        time_differential = TWO_WEEKS.div_euclid(4);
    }
    // the new target is the previous target * time differential / two weeks
    let mut new_target = (bits_to_target(&previous_bits) * time_differential).div_euclid(&(BigUint::from(TWO_WEEKS)));
    let max_target = BigUint::from_str_radix("ffff", 16).unwrap() * BigUint::from(pow(BigUint::from(256u32), 0x1d - 3));
    if new_target > max_target {
        new_target = max_target
    }

    target_to_bits(&new_target)
}
