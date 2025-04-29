use std::ops::Add;
use num::BigInt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PointScalar {
    x: Option<BigInt>,
    y: Option<BigInt>,
    a: BigInt,
    b: BigInt,
}
impl PointScalar {
    pub fn new(x: &Option<BigInt>, y: &Option<BigInt>, a: &BigInt, b: &BigInt) -> Self {
        if !x.is_none() && !y.is_none() {
            let x = x.clone().unwrap();
            let y = y.clone().unwrap();
            if y.pow(2) != x.pow(3) + a * x + b {
                panic!("Point is not on the curve");
            }
        }
        PointScalar {
            x: x.clone(),
            y: y.clone(),
            a: a.clone(),
            b: b.clone()
        }
    }
}
impl PointScalar {
    fn is_inf(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}
impl Add for PointScalar {
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
            PointScalar::new(
                &None,
                &None,
                &BigInt::from(self.a.clone()),
                &BigInt::from(self.b.clone()),
            )
        } else if self.x != other.x {
            let s = (other.y.clone().unwrap() - self.y.clone().unwrap())/(other.x.clone().unwrap() - self.x.clone().unwrap());
            let x = &s.pow(2) - self.x.clone().unwrap() - other.x.clone().unwrap();
            let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
            PointScalar::new(
                &Some(x.clone()),
                &Some(y.clone()),
                &BigInt::from(self.a.clone()),
                &BigInt::from(self.b.clone()),
            )
        } else if self == other {
            if self.y.clone().unwrap() == BigInt::from(0i32) {
                PointScalar::new(
                    &None,
                    &None,
                    &BigInt::from(self.a.clone()),
                    &BigInt::from(self.b.clone()),
                )
            } else {
                let s = (3*(self.x.clone().unwrap().pow(2)) + self.a.clone())/ (2 * self.y.clone().unwrap());
                let x = (&s * &s) - 2 * self.x.clone().unwrap();
                let y = s * (self.x.clone().unwrap() - &x) - self.y.clone().unwrap();
                PointScalar::new(
                    &Some(x),
                    &Some(y),
                    &BigInt::from(self.a.clone()),
                    &BigInt::from(self.b.clone()),
                )
            }
        } else {
            panic!("Point data in not valid");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn test_point_off() {
        let _p3 = PointScalar::new(
            &Some(BigInt::from(77i32)),
            &Some(BigInt::from(77i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
    }
    #[test]
    fn test_point_eq() {
        let p1 = PointScalar::new(
            &Some(BigInt::from(3i32)),
            &Some(BigInt::from(-7i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        assert!(p1 == p1);
        let p2 = PointScalar::new(
            &None,
            &None,
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        assert!(p2 == p2);
    }
    #[test]
    fn test_point_add_0() {
        let p1 = PointScalar::new(
            &None,
            &None,
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        let p2 = PointScalar::new(
            &Some(BigInt::from(2i32)),
            &Some(BigInt::from(5i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        let p3 = PointScalar::new(
            &Some(BigInt::from(2i32)),
            &Some(BigInt::from(-5i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        assert_eq!(p1.clone() + p2.clone(), p2.clone());
        assert_eq!(p2.clone() + p1.clone(), p2.clone());
        assert_eq!(p2.clone() + p3.clone(), p1.clone());
    }
    #[test]
    fn test_point_add_1() {
        let p1 = PointScalar::new(
            &Some(BigInt::from(3i32)),
            &Some(BigInt::from(7i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        let p2 = PointScalar::new(
            &Some(BigInt::from(-1i32)),
            &Some(BigInt::from(-1i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        let p3 = PointScalar::new(
            &Some(BigInt::from(2i32)),
            &Some(BigInt::from(-5i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        assert_eq!(p1 + p2, p3);
    }
    #[test]
    fn test_point_add_2() {
        let p1 = PointScalar::new(
            &Some(BigInt::from(-1i32)),
            &Some(BigInt::from(-1i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        let p2 = PointScalar::new(
            &Some(BigInt::from(18i32)),
            &Some(BigInt::from(77i32)),
            &BigInt::from(5i32),
            &BigInt::from(7i32),
        );
        assert_eq!(p1.clone() + p1, p2);
    }
}