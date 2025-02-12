use num::BigUint;
use num::Num;

pub struct Secp256k1 {
    pub A: BigUint,
    pub B: BigUint,
    pub P: BigUint,
    pub N: BigUint,
    pub GX: BigUint,
    pub GY: BigUint,
}
impl Secp256k1 {
    pub fn new() -> Secp256k1 {
        Secp256k1 {
            A: BigUint::from(0u32),
            B: BigUint::from(7u32),
            P: BigUint::from(2u32).pow(256u32) - BigUint::from(2u32).pow(32u32) - BigUint::from(977u32),
            N: BigUint::from_str_radix("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141", 16).unwrap(),
            GX: BigUint::from_str_radix("79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798", 16).unwrap(),
            GY: BigUint::from_str_radix("483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8", 16).unwrap(),
        }
    }
}
