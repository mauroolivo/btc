
use num::{BigUint, ToPrimitive};
use num::traits::Euclid;
use crate::helpers::hash256::hash256;

static BASE58_ALPHABET : &'static [u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn base58_encode(val: Vec<u8>) -> Vec<u8> {

    let mut leading_zeros = 0;

    for byte in &val {
        if *byte == 0 {
            leading_zeros += 1;
        } else {
            break;
        }
    }
    let mut num = BigUint::from_bytes_be(val.as_slice());

    let mut prefix : Vec<u8> = Vec::new();

    for _ in 0..leading_zeros {
        let c = BASE58_ALPHABET[0u32 as usize];
        // println!("{}", c);
        prefix.push(c);
    }

    let mut result : Vec<u8> = Vec::new();
    while num > BigUint::from(0u32) {
        let (div, rem) = num.div_rem_euclid(&BigUint::from(58u8));
        result.push(BASE58_ALPHABET[rem.to_u32().unwrap() as usize]);
        num = div;
    }
    result.reverse();
    let mut final_value = prefix;
    final_value.extend(result);
    final_value
}

pub fn base58_encode_checksum(val: Vec<u8>) -> Vec<u8> {
    let mut result = val.clone();
    let hash = hash256(val.as_slice());
    result.extend_from_slice(&hash[0..4]);
    base58_encode(result.to_vec())
}
pub fn decode_base58(val: Vec<u8>) -> Vec<u8> {
    let mut num = BigUint::from(0u32);
    for c in val {
        num *= BigUint::from(58u8);
        let index = BASE58_ALPHABET.iter().position(|&r| r == c).unwrap();
        num += BigUint::from(index);
    }
    let combined = num.to_bytes_be();
    let checksum = combined.as_slice()[combined.len() - 4..].to_vec();
    let rest = combined.as_slice()[..combined.len() - 4].to_vec();
    let hash = hash256(rest.as_slice());
    if hash[..4] != checksum {
        panic!("decode_base58 checksum mismatch");
    }
    let result = combined[1..combined.len() - 4].to_vec();
    result
}
#[cfg(test)]
mod tests {
    use std::io::Write;
    use super::*;
    use crate::helpers::hex::hex;
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
            let expected: Vec<u8> = expected.as_bytes().to_vec();
            assert_eq!(result, expected);
        }
    }
    #[test]
    fn encode_58_checksum() {
        let value = "7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d";
        let value: Vec<u8> = hex::decode(value).unwrap();
        let expected = "wdA2ffYs5cudrdkhFm5Ym94AuLvavacapuDBL2CAcvqYPkcvi";
        assert_eq!(base58_encode_checksum(value), expected.as_bytes().to_vec());
    }
    #[test]
    fn decode_58() {

        let addr = "mnrVtF8DWjMu839VW3rBfgYaAfKk8983Xf".as_bytes().to_vec();
        let h160 = hex(decode_base58(addr.clone()));

        let want = "507b27411ccf7f16f10297de6cef3f291623eddf";
        assert_eq!(want, h160);

        // prefix: 0x00 mainnet, 0x6f testnet
        let mut buffer = Vec::new();
        let bytes = hex::decode(h160).unwrap();

        let _ = buffer.write_all(&[0x6f]);
        let _ = buffer.write_all(&bytes);
        let got = base58_encode_checksum(buffer);
        assert_eq!(addr, got);
    }
}