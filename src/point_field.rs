use std::ops::{Add};
use num::BigUint;
use crate::field_element::FieldElement;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PointField {
    x: Option<FieldElement>,
    y: Option<FieldElement>,
    a: FieldElement,
    b: FieldElement,
}

impl PointField {
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
        PointField {
            x: x.clone(),
            y: y.clone(),
            a: a.clone(),
            b: b.clone()
        }
    }
    fn is_inf(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}
impl Add for PointField {
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
            PointField::new(
                &None,
                &None,
                &self.a.clone(),
                &self.b.clone(),
            )
        } else if self.x != other.x {
            let s = (other.y.clone().unwrap() - self.y.clone().unwrap())/(other.x.clone().unwrap() - self.x.clone().unwrap());
            let x = s.pow(2).clone() - self.x.clone().unwrap() - other.x.clone().unwrap();
            let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
            PointField::new(
                &Some(x.clone()),
                &Some(y.clone()),
                &self.a.clone(),
                &self.b.clone(),
            )
        } else if self == other {
            if (self.y.clone().unwrap()).num_value() == BigUint::from(0u32) {
                PointField::new(
                    &None,
                    &None,
                    &self.a.clone(),
                    &self.b.clone(),
                )
            } else {
                let s = ((self.x.clone().unwrap().pow(2)*BigUint::from(3u32)) + self.a.clone())/ (self.y.clone().unwrap()*BigUint::from(2u32));
                let x = (s.clone() * s.clone()) - self.x.clone().unwrap() * BigUint::from(2u32);
                let y = s * (self.x.clone().unwrap() - x.clone()) - self.y.clone().unwrap();
                PointField::new(
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
#[cfg(test)]
mod tests {
    use num::BigUint;
    use super::*;
    #[test]
    fn point1_on() {
        let p = 223u32;
        let _p3 = PointField::new(
            &Some(FieldElement::new(&BigUint::from(192u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(105u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );

    }
    #[test]
    fn point2_on() {
        let p = 223u32;
        let _p3 = PointField::new(
            &Some(FieldElement::new(&BigUint::from(17u32), &BigUint::from(p))),
            &Some(FieldElement::new(&BigUint::from(56u32), &BigUint::from(p))),
            &FieldElement::new(&BigUint::from(0u32), &BigUint::from(p)),
            &FieldElement::new(&BigUint::from(7u32), &BigUint::from(p))
        );

    }
    #[test]
    fn point3_on() {
        let p = 223u32;
        let _p3 = PointField::new(
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
        let _p3 = PointField::new(
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
        let _p3 = PointField::new(
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
            let p1 = PointField::new(&x1, &y1, &a, &b);
            let p2 = PointField::new(&x2, &y2, &a, &b);
            let p3 = PointField::new(&x3, &y3, &a, &b);
            assert_eq!(p1+p2, p3);
        }
    }
}