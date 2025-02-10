use std::ops::{Add, Div, Mul, Sub};
use num::{BigInt, BigUint, One};
use num::bigint::ToBigInt;
use num::traits::Euclid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FieldElement {
    num: BigUint,
    prime: BigUint
}
impl FieldElement {
    pub fn new(num: &BigUint, prime: &BigUint) -> Self {
        if num >= prime {
            panic!("num not in range 0-{}.", prime - BigUint::one() );
        }
        FieldElement {
            num: num.clone(),
            prime: prime.clone(),
        }
    }

    pub fn pow(&self, exponent: i32) -> Self {
        let one = BigInt::from(1u32);
        let p = &self.prime.to_bigint().unwrap();
        let exp = BigInt::from(exponent);
        // modulus for negative exponent
        // In C, C++, D, C#, F# and Java, % is in fact the remainder.
        // In Perl, Python or Ruby, % is the modulus
        let n = exp.rem_euclid(&(p - one));
        let num = self.num.modpow(&n.to_biguint().unwrap(), &self.prime);
        Self::new(&num, &self.prime)
    }
    // pub fn value(&self) -> BigUint {
    //     self.value
    // }
}


impl Add for FieldElement {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }
        let s = &self.num + &other.num;
        let mod_sum = s % &self.prime;

        Self::new(&mod_sum, &self.prime)
    }
}
impl Sub for FieldElement {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot subtract two numbers in different Fields");
        }
        let p = &self.prime;
        let a = &self.num % p;
        let b = &other.num % p;

        Self::new(&((a + p - b) % p), &self.prime)
    }
}
impl Mul for FieldElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields");
        }
        let prod = self.num * other.num;
        let mod_prod = prod % &self.prime;

        Self::new(&mod_prod, &self.prime)
    }
}
impl Div for FieldElement {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        if self.prime != other.prime {
            panic!("Cannot divide two numbers in different Fields");
        }

        // 1/b other ^ (p-2)
        let b_reciprocal = other.num.modpow(&(&self.prime - BigUint::from(2u32)), &self.prime);

        let n = (self.num * b_reciprocal) % &self.prime;
        Self::new(&n, &self.prime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_ne() {
        let p = 31u32;
        let a = FieldElement::new(&BigUint::from(2u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(2u32), &BigUint::from(p));
        let c = FieldElement::new(&BigUint::from(15u32), &BigUint::from(p));
        assert_eq!(a, b);
        assert_ne!(a, c);
        assert!(a != c);
        assert!(a == b)
    }
    #[test]
    fn add() {
        let p = 31u32;
        let a = FieldElement::new(&BigUint::from(2u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(15u32), &BigUint::from(p));
        assert_eq!(a+b, FieldElement::new(&BigUint::from(17u32), &BigUint::from(p)));
        let a = FieldElement::new(&BigUint::from(17u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(21u32), &BigUint::from(p));
        assert_eq!(a+b, FieldElement::new(&BigUint::from(7u32), &BigUint::from(p)));
    }
    #[test]
    fn sub() {
        let p = 31u32;
        let a = FieldElement::new(&BigUint::from(29u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(4u32), &BigUint::from(p));
        assert_eq!(a-b, FieldElement::new(&BigUint::from(25u32), &BigUint::from(p)));
        let a = FieldElement::new(&BigUint::from(15u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(30u32), &BigUint::from(p));
        assert_eq!(a-b, FieldElement::new(&BigUint::from(16u32), &BigUint::from(p)));
    }
    #[test]
    fn mul() {
        let p = 31u32;
        let a = FieldElement::new(&BigUint::from(24u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(19u32), &BigUint::from(p));
        assert_eq!(a*b, FieldElement::new(&BigUint::from(22u32), &BigUint::from(p)));
    }
    #[test]
    fn pow() {
        let p = 31u32;
        let a = FieldElement::new(&BigUint::from(17u32), &BigUint::from(p));
        assert_eq!(a.pow(3), FieldElement::new(&BigUint::from(15u32), &BigUint::from(p)));
        let a = FieldElement::new(&BigUint::from(5u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(18u32), &BigUint::from(p));
        assert_eq!(a.pow(5) * b, FieldElement::new(&BigUint::from(16u32), &BigUint::from(p)));
    }
    #[test]
    fn div() {
        let p = 31u32;
        let a = FieldElement::new(&BigUint::from(3u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(24u32), &BigUint::from(p));
        assert_eq!(a / b, FieldElement::new(&BigUint::from(4u32), &BigUint::from(p)));
    }
    #[test]
    fn neg_pow() {
        let p = 31u32;
        let a = FieldElement::new(&BigUint::from(17u32), &BigUint::from(p));
        assert_eq!(a.pow(-3), FieldElement::new(&BigUint::from(29u32), &BigUint::from(p)));
        let a = FieldElement::new(&BigUint::from(4u32), &BigUint::from(p));
        let b = FieldElement::new(&BigUint::from(11u32), &BigUint::from(p));
        assert_eq!(a.pow(-4) * b, FieldElement::new(&BigUint::from(13u32), &BigUint::from(p)));
    }
}
