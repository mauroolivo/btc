use std::io::Read;
use num::{BigUint, ToPrimitive};
use num::traits::Euclid;
use crate::helpers::hash256;
use crate::helpers::hash256::hash256;

static BASE58_ALPHABET : &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub fn base58_encode(val: Vec<u8>) -> Vec<u8> {
    let bytes = val.as_slice();

    let mut leading_zeros_count = 0;
    for byte in bytes {
        // println!("{:x}", byte);
        if *byte == 0 {
            leading_zeros_count += 1;
        } else {
            break;
        }
    }
    let mut num = BigUint::from_bytes_be(val.as_slice());

    let mut prefix : Vec<u8> = Vec::new();
    for _ in 0..leading_zeros_count {
        prefix.push(0x01);
    }

    let mut result : Vec<u8> = Vec::new();
    while num > BigUint::from(0u32) {
        let (q, rem) = num.div_rem_euclid(&BigUint::from(58u32));
        result.push(BASE58_ALPHABET[rem.to_usize().unwrap()]);
        num = q;
    }
    result.reverse();

    let mut final_value = prefix;
    final_value.extend(result);
    final_value
}

pub fn base58_encode_checksum(bytes: Vec<u8>) -> Vec<u8> {
    let mut result = bytes;
    let hash = hash256(&result);
    result.extend_from_slice(&hash[0..4]);
    base58_encode(result)
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn encode_58() {
        let values = vec![
            (
                "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d",
                "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6",
            ),
            (
                "eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c",
                "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd",
            ),
            (
                "c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6",
                "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7",
            ),
        ];

        for (value, expected) in values {
            let value: Vec<u8> = hex::decode(value).unwrap();
            let result = base58_encode(value);

            // let string = String::from_utf8(result.clone()).unwrap();
            assert_eq!(result, expected.as_bytes());
        }
    }
    #[test]
    fn encode_58_checksum() {
        let value = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let value: Vec<u8> = hex::decode(value).unwrap();
        let expected = "wdA2ffYs5cudrdkhFm5Ym94AuLvavacapuDBL2CAcvqYPkcvi";
        assert_eq!(base58_encode_checksum(value.clone()), expected.as_bytes());
    }
}