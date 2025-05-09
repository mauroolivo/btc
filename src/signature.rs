use core::fmt;
use std::io::{Cursor, Error};
use num::{BigUint};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signature {
    r: BigUint,
    s: BigUint,
}

impl Signature {
    pub fn new(r: &BigUint, s: &BigUint) -> Self {
        Self {
            r: r.clone(),
            s: s.clone(),
        }
    }
    pub fn r(&self) -> &BigUint {
        &self.r
    }
    pub fn s(&self) -> &BigUint {
        &self.s
    }

    pub fn parse(der: &Vec<u8>) -> Result<Signature, Error> {

        let mut buffer = [0u8; 1];
        let mut stream = &mut Cursor::new(der);
        //stream.read(&mut buffer)?;
        std::io::Read::read(&mut stream, &mut buffer)?;
        let compound = buffer[0];
        if compound != 0x30 {
            panic!("Invalid signature compound");
        }
        let mut buffer = [0u8; 1];
        std::io::Read::read(&mut stream, &mut buffer)?;
        let length = buffer[0];
        if length as usize + 2usize != der.len() {
            panic!("Invalid signature length");
        }
        let mut buffer = [0u8; 1];
        std::io::Read::read(&mut stream, &mut buffer)?;
        let marker = buffer[0];
        if marker != 0x02 {
            panic!("Invalid signature marker");
        }
        let mut buffer = [0u8; 1];
        std::io::Read::read(&mut stream, &mut buffer)?;
        let rlength = buffer[0];

        let mut buffer = vec![0u8; rlength as usize];

        std::io::Read::read(&mut stream, &mut buffer)?;
        let r = BigUint::from_bytes_be(buffer.as_slice());

        let mut buffer = [0u8; 1];
        std::io::Read::read(&mut stream, &mut buffer)?;
        let marker = buffer[0];
        if marker != 0x02 {
            panic!("Invalid signature marker 2");
        }
        std::io::Read::read(&mut stream, &mut buffer)?;
        let slength = buffer[0];
        let mut buffer = vec![0u8; slength as usize];

        std::io::Read::read(&mut stream, &mut buffer)?;
        let s = BigUint::from_bytes_be(buffer.as_slice());

        if der.len() != 6usize + rlength as usize  + slength as usize {
            panic!("Signature too long");
        }
        Ok(Signature::new(&r, &s))
    }
}
impl Signature {
    pub fn der(&self) -> Vec<u8> {
        let mut rbin = self.r().to_bytes_be();

        while rbin.len() > 0 && rbin[0] == 0x00 {
            rbin.pop();
        }

        let rbin =
            if rbin.len() > 0 && rbin[0] & 0x80 > 0 {
                let mut value : Vec<u8> = vec![0x00];
                value.extend(rbin);
                value
            } else {
                rbin
            };

        let mut str = vec![0x02, rbin.len() as u8];
        str.extend(rbin); //result.extend_from_slice(&rbin);

        let mut sbin = self.s.to_bytes_be();

        while !sbin.len() > 0 && sbin[0] == 0x00 {
            sbin.pop();
        }

        let sbin =
            if sbin.len() > 0 && sbin[0] & 0x80 > 0 {
                let mut value : Vec<u8> = vec![0x00];
                value.extend(sbin);
                value
            } else {
                sbin
            };

        str.extend(vec![0x02, sbin.len() as u8]);
        str.extend(sbin);

        let mut res : Vec<u8> = vec![0x30, str.len() as u8];
        res.extend(str);
        res
    }
}
impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Signature({:x},{:x})", self.r, self.s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num::Num;

    #[test]
    fn test_der() {
        let r = BigUint::from_str_radix(
            "37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        ).unwrap();

        let s = BigUint::from_str_radix(
            "8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        ).unwrap();

        let sig = Signature::new(&r, &s);

        assert_eq!(
            sig.der(),
            hex::decode(
                "3045022037206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6\
                0221008ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec"
            ).unwrap()
        );
    }
}