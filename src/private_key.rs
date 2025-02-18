use num::BigUint;
use sha2::{Digest, Sha256};
use crate::signature::Signature;
use crate::field_element::FieldElement;
use crate::point::Point;
use crate::secp256k1::Secp256k1;
use rfc6979::consts::U32;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PrivateKey {
    secret: BigUint,
    public_key: Point,
}

impl PrivateKey {
    pub fn new(secret: &BigUint) -> Self {
        let s256 = Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let public_key = generator * secret.clone();
        PrivateKey {
            secret: secret.clone(),
            public_key: public_key.clone(),
        }
    }

    fn sign(&self, z: &BigUint) -> Signature {

        let k = self.deterministic_k(&z);

        let s256 = Secp256k1::new();
        let n = &s256.n;

        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let r = (generator * k.clone()).x().unwrap().num_value();
        let k_inv = k.modpow(&(n - &BigUint::from(2u8)), &n);

        //let sig = (z + r * &self.secret) * k_inv % n;
        let mut s = (z + &r * &self.secret) * k_inv % n;

        if s > n / BigUint::from(2u8) {
            s = n - s;
        }
        Signature::new(&r, &s)
    }

    fn deterministic_k(&self, z: &BigUint) -> BigUint {
        let s256 = Secp256k1::new();
        let n_bytes = s256.n.to_bytes_be();
        let mut n = [0; 32];
        //let mut p = GenericArray::<u8, 32>::default();
        n.copy_from_slice(&n_bytes);

        let k_bytes = self.secret.to_bytes_be();
        let mut k = [0; 32];
        k.copy_from_slice(&k_bytes);

        let z_bytes = z.to_bytes_be();
        let mut z = [0; 32];
        z.copy_from_slice(&z_bytes);

        let h = Sha256::digest(&z);

        // secret, field modulus, hash/digest (modulus reduced), additional data
        let k = rfc6979::generate_k::<Sha256, U32>(&k.into(), &n.into(), &h, b"");
            //<Sha256, u32>();

        BigUint::from_bytes_be(&k)
    }
}

#[cfg(test)]
mod tests {
    use num::bigint::Sign;
    use num::BigUint;
    use crate::helpers::hash256::hash256;
    use super::*;

    #[test]
    fn sign() {
        let s256 = Secp256k1::new();
        let n = s256.n;
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));

        let hash = hash256(b"my secret");
        let e = BigUint::from_bytes_be(hash.as_slice());

        let private_key = PrivateKey::new(&e);

        let hash = hash256(b"my message");
        let z = BigUint::from_bytes_be(hash.as_slice());

        let sig = private_key.sign(&z);

        let point = generator * e;

        assert!(point.verify(&z, &sig));
    }
}
