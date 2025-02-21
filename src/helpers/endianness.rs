use num::{BigUint, ToPrimitive};
use std::io::{Write};
pub fn little_endian_to_int(bytes: &[u8]) -> BigUint {
    BigUint::from_bytes_le(bytes)
}
pub fn int_to_little_endian(n: BigUint, length: u32) -> Vec<u8> {
    let mut buffer = Vec::new();
    if length == 1 {
        let _ = buffer.write_all(&n.to_u8().unwrap().to_le_bytes());
        buffer
    } else if length == 2 {
        let _ = buffer.write_all(&n.to_u16().unwrap().to_le_bytes());
        buffer
    } else if length == 4 {
        let _ = buffer.write_all(&n.to_u32().unwrap().to_le_bytes());
        buffer
    } else if length == 8 {
        let _ = buffer.write_all(&n.to_u64().unwrap().to_le_bytes());
        buffer
    } else {
        panic!("Unsupported little_endian length");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn le_to_int_1() {
        let h = "99c3980000000000";
        let decoded = hex::decode(h).unwrap();
        let expect = BigUint::from(10011545u32);
        assert_eq!(little_endian_to_int(decoded.as_slice()), expect);
    }
    #[test]
    fn le_to_int_2() {
        let h = "a135ef0100000000";
        let decoded = hex::decode(h).unwrap();
        let expect = BigUint::from(32454049u32);
        assert_eq!(little_endian_to_int(decoded.as_slice()), expect);
    }
    #[test]
    fn int_to_le_1() {
        let n = BigUint::from(1u32);
        let expect = b"\x01\x00\x00\x00";
        let res = int_to_little_endian(n,4);
        println!("{:?}", res.as_slice());
        assert_eq!(res, expect);
    }
    #[test]
    fn int_to_le_2() {
        let n = BigUint::from(10011545u32);
        let expect = b"\x99\xc3\x98\x00\x00\x00\x00\x00";
        let res = int_to_little_endian(n,8);
        println!("{:?}", res.as_slice());
        assert_eq!(res, expect);
    }
}
