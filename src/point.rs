use std::ops::{Add, Mul};
use num::{BigInt, BigUint, Integer};
use std::{fmt};
use crate::field_element::FieldElement;
use crate::secp256k1;
use crate::signature::Signature;
use crate::helpers::hash160::hash160;
use crate::helpers::base58::base58_encode_checksum;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl Point {
    pub fn new(x: &Option<FieldElement>, y: &Option<FieldElement>, a: &FieldElement, b: &FieldElement) -> Self {
        if !x.is_none() && !y.is_none() {
            let x = x.clone().unwrap();
            let y = y.clone().unwrap();
            let a = a.clone();
            let b = b.clone();
            if y.pow(BigInt::from(2u32)) != x.pow(BigInt::from(3u32)) + a * x + b {
                panic!("Point is not on the curve");
            }
        }
        Point {
            x: x.clone(),
            y: y.clone(),
            a: a.clone(),
            b: b.clone()
        }
    }
    pub fn new_secp256k1(x: &Option<FieldElement>, y: &Option<FieldElement>) -> Self {
        let s = secp256k1::Secp256k1::new();
        let a = FieldElement::new(&s.a, &s.p);
        let b = FieldElement::new(&s.b, &s.p);
        if !x.is_none() && !y.is_none() {
            let x = x.clone().unwrap();
            let y = y.clone().unwrap();
            if y.pow(BigInt::from(2u32)) != x.pow(BigInt::from(3u32)) + a.clone() * x + b.clone() {
                panic!("Point is not on the curve");
            }
        }
        Point {
            x: x.clone(),
            y: y.clone(),
            a: a,
            b: b
        }
    }
    pub fn verify(&self, z: &BigUint, signature: &Signature) -> bool {
        let s256 = secp256k1::Secp256k1::new();

        let n = &s256.n;
        let s = signature.s();
        let two = &BigUint::from(2u8);
        let s_inv = s.modpow(&(n - two), &n);

        // u = z / s
        let u = z * &s_inv % n;

        // v = r / s
        let v = signature.r() * &s_inv % n;

        // u*G + v*P should have as the x coordinate, r
        let s = secp256k1::Secp256k1::new();
        let g = Point::new_secp256k1(
            &Some(FieldElement::new(&s.gx, &s.p)),
            &Some(FieldElement::new(&s.gy, &s.p))
        );
        let p = self.clone();
        let total = g * u + p * v;

        total.x.unwrap().num_value() == signature.r().clone()
    }
    fn is_inf(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
    pub fn x(&self) -> Option<FieldElement> {
        self.x.clone()
    }
    pub fn sec(&self, compressed: bool) -> Vec<u8> {
        let mut sec : Vec<u8> = Vec::new();
        if compressed {
            if self.y.clone().unwrap().num_value() % BigUint::from(2u32) == BigUint::from(0u32) {
                sec.push(0x02);
                sec.extend(self.x.clone().unwrap().num_value().to_bytes_be());
                sec
            } else {
                sec.push(0x03);
                sec.extend(self.x.clone().unwrap().num_value().to_bytes_be());
                sec
            }
        } else {
            sec.push(0x04);
            sec.extend(self.x.clone().unwrap().num_value().to_bytes_be());
            sec.extend(self.y.clone().unwrap().num_value().to_bytes_be());
            sec
        }
    }
    pub fn address(&self, compressed: bool, testnet: bool) -> String {
        let sec = self.sec(compressed);

        let h160 = hash160(&sec.as_slice());
        let prefix = if testnet { b"\x6f" } else { b"\x00" };

        let mut address = prefix.to_vec();


        address.extend(h160);
        println!("--- {:?}", address);
        let to_retrun = base58_encode_checksum(address.as_slice());

        println!("{:?}", to_retrun);
        to_retrun

        // left: "\u{1}F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1"
        // right: "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1"

    }
    pub fn parse(data: &[u8]) -> Self {
        let s256 = secp256k1::Secp256k1::new();
        if data[0] == 0x04 { // uncompressed
            let x = BigUint::from_bytes_be(&data[1..33]);
            let y = BigUint::from_bytes_be(&data[33..65]);

            return Self::new_secp256k1(
                &Some(FieldElement::new(&x, &s256.p)),
                &Some(FieldElement::new(&y, &s256.p)),
            );
        }

        let is_even = data[0] == 0x02;
        let x = BigUint::from_bytes_be(&data[1..]);
        let x = FieldElement::new(&x, &s256.p);

        // right side of the equation y^2 = x^3 + 7
        let alpha = x.pow(BigInt::from(3u32)) + FieldElement::new(&s256.b, &s256.p);

        // solve for left side
        let beta = alpha.sqrt();

        let even_beta = if beta.num_value().is_even() {
            beta.clone()
        } else {
            FieldElement::new(&(&s256.p - beta.clone().num_value()), &s256.p)
        };

        let odd_beta = if beta.num_value().is_even() {
            FieldElement::new(&(&s256.p - beta.clone().num_value()), &s256.p)
        } else {
            beta.clone()
        };

        if is_even {
            Self::new_secp256k1(&Some(x), &Some(even_beta))
        } else {
            Self::new_secp256k1(&Some(x), &Some(odd_beta))
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.x.as_ref(), self.y.as_ref()) {
            (Some(x), Some(y)) => write!(f, "({:x}, {:x})", x.num_value(), y.num_value()),
            (Some(x), None) => write!(f, "({:x}, ∞)", x.num_value()),
            (None, Some(y)) => write!(f, "(∞, {:x})", y.num_value()),
            _ => write!(f, "(∞, ∞)")
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        if self.a != other.a || self.b != other.b {
            panic!("Points are not on the same curve");
        }
        if self.is_inf() {
            return other.clone();
        }
        if other.is_inf() {
            return self.clone();
        }
        if self.x == other.x && self.y != other.y {
            Point::new(
                &None,
                &None,
                &self.a.clone(),
                &self.b.clone(),
            )
        } else if self.x != other.x {
            let s = (other.y.clone().unwrap() - self.y.clone().unwrap())/(other.x.clone().unwrap() - self.x.clone().unwrap());
            let x = s.pow(BigInt::from(2u32)).clone() - self.x.clone().unwrap() - other.x.clone().unwrap();
            let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
            Point::new(
                &Some(x.clone()),
                &Some(y.clone()),
                &self.a.clone(),
                &self.b.clone(),
            )
        } else if self == other {
            if (self.y.clone().unwrap()).num_value() == BigUint::from(0u32) {
                Point::new(
                    &None,
                    &None,
                    &self.a.clone(),
                    &self.b.clone(),
                )
            } else {
                let s = ((self.x.clone().unwrap().pow(BigInt::from(2u32))*BigUint::from(3u32)) + self.a.clone())/ (self.y.clone().unwrap()*BigUint::from(2u32));
                let x = (s.clone() * s.clone()) - self.x.clone().unwrap() * BigUint::from(2u32);
                let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
                Point::new(
                    &Some(x),
                    &Some(y),
                    &self.a.clone(),
                    &self.b.clone(),
                )
            }
        } else {
            panic!("Point data in not valid");
        }
    }
}

impl Mul<BigUint> for Point {
    type Output = Self;

    fn mul(self, coefficient: BigUint) -> Self {
        let mut coef = coefficient;
        let mut current = self.clone();
        // We start the result at 0, or the point at infinity.
        let mut result = Point::new(
            &None,
            &None,
            &self.a.clone(),
            &self.b.clone(),
        );

        while coef > BigUint::from(0u32) {
            if &coef & BigUint::from(1u32) == BigUint::from(1u32) {
                result = result + current.clone();
            }
            current = current.clone() + current.clone();
            coef >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use num::BigUint;
    use num::Num;
    use super::*;
    #[test]
    fn point1_on() {
        let p = 223u32;
        let _p3 = Point::new(
            &Some(FieldElement::new(&BigUint::from(192u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(105u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );

    }
    #[test]
    fn point2_on() {
        let p = 223u32;
        let _p3 = Point::new(
            &Some(FieldElement::new(&BigUint::from(17u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(56u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );

    }
    #[test]
    fn point3_on() {
        let p = 223u32;
        let _p3 = Point::new(
            &Some(FieldElement::new(&BigUint::from(1u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(193u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );

    }
    #[test]
    #[should_panic]
    fn point1_off() {
        let p = 223u32;
        let _p3 = Point::new(
            &Some(FieldElement::new(&BigUint::from(200u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(119u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );
    }
    #[test]
    #[should_panic]
    fn point2_off() {
        let p = 223u32;
        let _p3 = Point::new(
            &Some(FieldElement::new(&BigUint::from(42u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(99u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );
    }
    #[test]
    fn add() {
        let p = 223u32;
        let a = FieldElement::new(&BigUint::from(0u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(7u32), &BigUint::from(p));

        let mut vectors:Vec<Vec<u32>> = Vec::new();
        let mut vec:Vec<u32> = Vec::new();
        vec.push(192);
        vec.push(105);
        vec.push(17);
        vec.push(56);
        vec.push(170);
        vec.push(142);
        vectors.push(vec.clone());
        let mut vec:Vec<u32> = Vec::new();
        vec.push(47);
        vec.push(71);
        vec.push(117);
        vec.push(141);
        vec.push(60);
        vec.push(139);
        vectors.push(vec.clone());
        let mut vec:Vec<u32> = Vec::new();
        vec.push(143);
        vec.push(98);
        vec.push(76);
        vec.push(66);
        vec.push(47);
        vec.push(71);
        vectors.push(vec.clone());
        for vec in vectors {
            let x1 = Some(FieldElement::new(&BigUint::from(vec[0]), &BigUint::from(p)));
            let y1 = &Some(FieldElement::new(&BigUint::from(vec[1]), &BigUint::from(p)));
            let x2 = Some(FieldElement::new(&BigUint::from(vec[2]), &BigUint::from(p)));
            let y2 = &Some(FieldElement::new(&BigUint::from(vec[3]), &BigUint::from(p)));
            let x3 = Some(FieldElement::new(&BigUint::from(vec[4]), &BigUint::from(p)));
            let y3 = &Some(FieldElement::new(&BigUint::from(vec[5]), &BigUint::from(p)));
            let p1 = Point::new(&x1, &y1, &a, &b);
            let p2 = Point::new(&x2, &y2, &a, &b);
            let p3 = Point::new(&x3, &y3, &a, &b);
            assert_eq!(p1+p2, p3);
        }
    }
    #[test]
    fn scalar_mul_1() {
        let p = 223u32;
        let point = Point::new(
            &Some(FieldElement::new(&BigUint::from(15u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(86u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );
        let p_inf = Point::new(
            &None,
            &None,
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );
        assert_eq!(p_inf, point * BigUint::from(7u32));
    }
    #[test]
    fn scalar_mul_2() {
        let p = 223u32;
        let point = Point::new(
            &Some(FieldElement::new(&BigUint::from(47u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(71u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );
        let point2 = Point::new(
            &Some(FieldElement::new(&BigUint::from(47u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(152u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );
        assert_eq!(point2, point * BigUint::from(20u32));
    }
    #[test]
    fn new_secp256k1() {
        let s = secp256k1::Secp256k1::new();
        let _p = Point::new_secp256k1(
            &Some(FieldElement::new(&s.gx, &s.p)),
            &Some(FieldElement::new(&s.gy, &s.p))
        );
    }
    #[test]
    fn ord() {
        let s = secp256k1::Secp256k1::new();
        let p = Point::new_secp256k1(
            &Some(FieldElement::new(&s.gx, &s.p)),
            &Some(FieldElement::new(&s.gy, &s.p))
        );
        let _p_inf = Point::new_secp256k1(
            &None,
            &None
        );
        println!("{}", p);
        //assert_eq!(p * s.N, p_inf)
    }
    #[test]
    fn verify1() {
        let s = secp256k1::Secp256k1::new();
        let p = Point::new_secp256k1(
            &Some(FieldElement::new(&BigUint::from_str_radix("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c", 16).unwrap(), &s.p)),
            &Some(FieldElement::new(&BigUint::from_str_radix("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34", 16).unwrap(), &s.p)),
        );
        let z= BigUint::from_str_radix("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60", 16).unwrap();
        let r= BigUint::from_str_radix("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395", 16).unwrap();
        let s= BigUint::from_str_radix("68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4", 16).unwrap();

        let sig = Signature::new(&r, &s);

        assert_eq!(p.verify(&z, &sig), true)

    }
    #[test]
    fn verify2() {
        let s = secp256k1::Secp256k1::new();
        let p = Point::new_secp256k1(
            &Some(FieldElement::new(&BigUint::from_str_radix("887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c", 16).unwrap(), &s.p)),
            &Some(FieldElement::new(&BigUint::from_str_radix("61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34", 16).unwrap(), &s.p)),
        );
        let z= BigUint::from_str_radix("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d", 16).unwrap();
        let r= BigUint::from_str_radix("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c", 16).unwrap();
        let s= BigUint::from_str_radix("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6", 16).unwrap();

        let sig = Signature::new(&r, &s);

        assert_eq!(p.verify(&z, &sig), true)
    }
    #[test]
    fn sec_1() {
        let s256 = secp256k1::Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));

        let k = BigUint::from(999u32).pow(3u32);
        let sec_uncompressed = b"049d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d56fa15cc7f3d38cda98dee2419f415b7513dde1301f8643cd9245aea7f3f911f9";
        let sec_compressed = b"039d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d5";
        let point = generator.clone() * k;

        let sec = point.sec(false);
        assert_eq!(sec, hex::decode(sec_uncompressed).unwrap());

        let sec = point.sec(true);
        assert_eq!(sec, hex::decode(sec_compressed).unwrap());

        let k = BigUint::from(999u32).pow(3u32);
        let sec_uncompressed = "049d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d56fa15cc7f3d38cda98dee2419f415b7513dde1301f8643cd9245aea7f3f911f9";
        let sec_compressed = "039d5ca49670cbe4c3bfa84c96a8c87df086c6ea6a24ba6b809c9de234496808d5";
        let point = generator.clone() * k;

        let sec = point.sec(false);
        assert_eq!(sec, hex::decode(sec_uncompressed).unwrap());

        let sec = point.sec(true);
        assert_eq!(sec, hex::decode(sec_compressed).unwrap());

        let k = BigUint::from(123u32);
        let sec_uncompressed = "04a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5204b5d6f84822c307e4b4a7140737aec23fc63b65b35f86a10026dbd2d864e6b";
        let sec_compressed = "03a598a8030da6d86c6bc7f2f5144ea549d28211ea58faa70ebf4c1e665c1fe9b5";
        let point = generator.clone() * k;

        let sec = point.sec(false);
        assert_eq!(sec, hex::decode(sec_uncompressed).unwrap());

        let sec = point.sec(true);
        assert_eq!(sec, hex::decode(sec_compressed).unwrap());

        let k = BigUint::from(42424242u32);
        let sec_uncompressed = "04aee2e7d843f7430097859e2bc603abcc3274ff8169c1a469fee0f20614066f8e21ec53f40efac47ac1c5211b2123527e0e9b57ede790c4da1e72c91fb7da54a3";
        let sec_compressed = "03aee2e7d843f7430097859e2bc603abcc3274ff8169c1a469fee0f20614066f8e";
        let point = generator.clone() * k;

        let sec = point.sec(false);
        assert_eq!(sec, hex::decode(sec_uncompressed).unwrap());

        let sec = point.sec(true);
        assert_eq!(sec, hex::decode(sec_compressed).unwrap());
    }
    #[test]
    fn sec_2() {
        let s256 = secp256k1::Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let point = generator.clone() * BigUint::from(5000u32);
        assert_eq!(
            point.sec(false),
            hex::decode(
                "04\
                ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4\
                f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10"
            )
                .unwrap()
        );
        let point = generator.clone() * BigUint::from(2018_u32).pow(5);
        assert_eq!(
            point.sec(false),
            hex::decode(
                "04\
                027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023\
                c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06"
            )
                .unwrap()
        );
        let point = generator.clone() * BigUint::from_str_radix("deadbeef12345", 16).unwrap();
        assert_eq!(
            point.sec(false),
            hex::decode(
                "04\
                d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd5\
                9961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121"
            )
                .unwrap()
        );
    }
    #[test]
    fn sec_3() {
        let s256 = secp256k1::Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let point = generator.clone() * BigUint::from(5001u32);
        assert_eq!(
            point.sec(true),
            hex::decode("0357a4f368868a8a6d572991e484e664810ff14c05c0fa023275251151fe0e53d1")
                .unwrap()
        );
        let point = generator.clone() * BigUint::from(2019_u32).pow(5);
        assert_eq!(
            point.sec(true),
            hex::decode("02933ec2d2b111b92737ec12f1c5d20f3233a0ad21cd8b36d0bca7a0cfa5cb8701")
                .unwrap()
        );
        let point = generator.clone() * BigUint::from_str_radix("deadbeef54321", 16).unwrap();
        assert_eq!(
            point.sec(true),
            hex::decode("0296be5b1292f6c856b3c5654e886fc13511462059089cdf9c479623bfcbe77690")
                .unwrap()
        );
    }
    #[test]
    fn sec_4() {
        let s256 = secp256k1::Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let point = generator.clone() * BigUint::from(5000u32);
        assert_eq!(Point::parse(&point.sec(false)), point);
        let point = generator.clone() * BigUint::from(2018_u32).pow(5);
        assert_eq!(Point::parse(&point.sec(false)), point);
        let point = generator.clone() * BigUint::from_str_radix("deadbeef12345", 16).unwrap();
        assert_eq!(Point::parse(&point.sec(false)), point);
    }
    #[test]
    fn sec_5() {
        let s256 = secp256k1::Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let point = generator.clone() * BigUint::from(5001u32);
        assert_eq!(Point::parse(&point.sec(true)), point);
        let point = generator.clone() * BigUint::from(2019_u32).pow(5);
        assert_eq!(Point::parse(&point.sec(true)), point);
        let point = generator.clone() * BigUint::from_str_radix("deadbeef54321", 16).unwrap();
        assert_eq!(Point::parse(&point.sec(true)), point);
    }
    #[test]
    fn address_1() {
        let s256 = secp256k1::Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));
        let point = generator.clone() * BigUint::from(5002u32);

        assert_eq!(
            point.address(false, true),
            "mmTPbXQFxboEtNRkwfh6K51jvdtHLxGeMA"
        );

        let point = generator.clone() * BigUint::from(2020_u32).pow(5);
        assert_eq!(
            point.address(true, true),
            "mopVkxp8UhXqRYbCYJsbeE1h1fiF64jcoH"
        );
        let point = generator.clone() * BigUint::from_str_radix("12345deadbeef", 16).unwrap();
        assert_eq!(
            point.address(true, false),
            "1F1Pn2y6pDb68E5nYJJeba4TLg2U7B6KF1"
        );
    }
    #[test]
    fn address_2() {
        let s256 = secp256k1::Secp256k1::new();
        let generator = Point::new_secp256k1(&Some(FieldElement::new(&s256.gx, &s256.p)), &Some(FieldElement::new(&s256.gy, &s256.p)));

        let secret1 = 888_u32.pow(3);
        let secret2 = 321_u32;
        let secret3 = 4242424242_u32;

        let values = vec![

            (secret1, true, false, "148dY81A9BmdpMhvYEVznrM45kWN32vSCN"),
            (secret1, true, true, "mieaqB68xDCtbUBYFoUNcmZNwk74xcBfTP"),
            (secret2, false, false, "1S6g2xBJSED7Qr9CYZib5f4PYVhHZiVfj"),
            (secret2, false, true, "mfx3y63A7TfTtXKkv7Y6QzsPFY6QCBCXiP"),
            (secret3, false, false, "1226JSptcStqn4Yq9aAmNXdwdc2ixuH9nb"),
            (secret3, false, true, "mgY3bVusRUL6ZB2Ss999CSrGVbdRwVpM8s"),
        ];

        for (secret, compressed, testnet, address) in values {
            let point = generator.clone() * BigUint::from(secret);
            assert_eq!(point.address(compressed, testnet), address);
        }
    }
}