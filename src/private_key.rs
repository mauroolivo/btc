use num::BigUint;
use sha2::{Digest, Sha256};
use crate::signature::Signature;
use rand::Rng;
use crate::field_element::FieldElement;
use crate::point::Point;
use crate::secp256k1::Secp256k1;

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

    fn sign(&self, z: &BigUint, k: &BigUint) -> Signature {
        let s256 = Secp256k1::new();
        let n = &s256.n;

        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let r = (generator * k.clone()).x().unwrap().num_value();
        let k_inv = k.modpow(&(n - &BigUint::from(2u8)), &n);

        //let sig = (z + r * &self.secret) * k_inv % n;
        let mut s = (z + &r * &self.secret) * k_inv % n;
        println!("s before -> {:x}", s);
        if s > n / BigUint::from(2u8) {
            s = n - s;
            println!("s before -> {:x}", s);
        }
        println!("z -> {:x}", z);
        Signature::new(&r, &s)
    }
}

#[cfg(test)]
mod tests {
    use num::bigint::Sign;
    use num::BigUint;
    use super::*;
    #[test]
    fn sign() {
        let has256 = Sha256::digest(Sha256::digest(&b"my message"));
        let z = BigUint::from_bytes_be(has256.as_slice());

        let has256 = Sha256::digest(Sha256::digest(&b"my secret"));
        let e = BigUint::from_bytes_be(has256.as_slice());

        let private_key = PrivateKey::new(&e);

        let k = &BigUint::from(1234567890u32);

        let s = private_key.sign(&z, &k);

        println!("signature -> {}", s);
    }
}
/*def hex(self):
return '{:x}'.format(self.secret).zfill(64)
# end::source13[]

# tag::source14[]
def sign(self, z):
k = self.deterministic_k(z)  # <1>
r = (k * G).x.num
k_inv = pow(k, N - 2, N)
s = (z + r * self.secret) * k_inv % N
if s > N / 2:
s = N - s
return Signature(r, s)

def deterministic_k(self, z):
k = b'\x00' * 32
v = b'\x01' * 32
if z > N:
z -= N
z_bytes = z.to_bytes(32, 'big')
secret_bytes = self.secret.to_bytes(32, 'big')
s256 = hashlib.sha256
k = hmac.new(k, v + b'\x00' + secret_bytes + z_bytes, s256).digest()
v = hmac.new(k, v, s256).digest()
k = hmac.new(k, v + b'\x01' + secret_bytes + z_bytes, s256).digest()
v = hmac.new(k, v, s256).digest()
while True:
v = hmac.new(k, v, s256).digest()
candidate = int.from_bytes(v, 'big')
if candidate >= 1 and candidate < N:
return candidate  # <2>
k = hmac.new(k, v + b'\x00', s256).digest()
v = hmac.new(k, v, s256).digest()
# end::source14[]*/