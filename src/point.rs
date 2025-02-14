use std::ops::{Add, Mul};
use num::BigUint;
use num::Num;
use std::fmt;
use crate::field_element::FieldElement;
use crate::secp256k1;
use crate::signature::Signature;

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
            if y.pow(2) != x.pow(3) + a * x + b {
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
            if y.pow(2) != x.pow(3) + a.clone() * x + b.clone() {
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
            let x = s.pow(2).clone() - self.x.clone().unwrap() - other.x.clone().unwrap();
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
                let s = ((self.x.clone().unwrap().pow(2)*BigUint::from(3u32)) + self.a.clone())/ (self.y.clone().unwrap()*BigUint::from(2u32));
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
    use num::bigint::Sign;
    use num::BigUint;
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
        let p = Point::new_secp256k1(
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
        let p_inf = Point::new_secp256k1(
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
}