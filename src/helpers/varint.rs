use std::io::{Cursor, Read, Write};
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};

pub fn read_varint(stream: &mut Cursor<Vec<u8>>) -> Result<u64, std::io::Error> {
    let mut buffer = [0; 1];
    stream.read(&mut buffer)?;
    let i = buffer[0];

    match i {
        // 0xfd > 2 bytes
        0xfd => {
            let mut buffer = [0; 2];
            stream.read(&mut buffer)?;
            Ok(little_endian_to_int(buffer.as_slice()).to_u64().unwrap())
        }
        // 0xfe > 4 bytes
        0xfe => {
            let mut buffer = [0; 4];
            stream.read(&mut buffer)?;
            Ok(little_endian_to_int(buffer.as_slice()).to_u64().unwrap())
        }
        // 0xff 8 bytes
        0xff => {
            let mut buffer = [0; 8];
            stream.read(&mut buffer)?;
            Ok(little_endian_to_int(buffer.as_slice()).to_u64().unwrap())
        }
        // the integer
        _ => Ok(u64::from(i)),
    }
}
pub fn encode_varint(i: u64) -> Result<Vec<u8>, std::io::Error> {
    let mut buffer = Vec::new();
    if i < 0xfd {
        // If the number is below 253, encode that number as a single byte (e.g.,
        // 100 → 0x64).
        buffer.write(int_to_little_endian(BigUint::from(i),1).as_slice())?;
    } else if i >= 0xfd && i <= 0xffff {
        buffer.write_all(&[0xfd])?;
        buffer.write(int_to_little_endian(BigUint::from(i),2).as_slice())?;
    } else if i >= 0x10000 && i <= 0xffffffff {
        buffer.write_all(&[0xfe])?;
        buffer.write(int_to_little_endian(BigUint::from(i),4).as_slice())?;
    } else if i >= 0x100000000 && i <= (u64::MAX - 1) {
        buffer.write_all(&[0xff])?;
        buffer.write(int_to_little_endian(BigUint::from(i),8).as_slice())?;
    } else {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Integer too large",
        ));
    }
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::helpers::hex::hex;
    use super::*;
    #[test]
    fn varint_1() {
        let test_cases = vec![
            (vec![0x01], 1),
            (vec![0xfd, 0x02, 0x00], 2),
            (vec![0xfe, 0x03, 0x00, 0x00, 0x00], 3),
            (
                vec![0xff, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                4,
            ),
            (
                vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff],
                18446744073709551615,
            ),
        ];

        for (input, expected_output) in test_cases {
            let mut cursor = Cursor::new(input);
            let result = read_varint(&mut cursor).unwrap();
            assert_eq!(result, expected_output);
        }
    }
    #[test]
    fn varint_2() {
        let test_cases = vec![
            (100, vec![0x64]),
            (255, vec![0xfd, 0xff, 0x00]),
            (555, vec![0xfd, 0x2b, 0x02]),
            (70015, vec![0xfe, 0x7f, 0x11, 0x01, 0x00]),
            (
                18005558675309,
                vec![0xff, 0x6d, 0xc7, 0xed, 0x3e, 0x60, 0x10, 0x00, 0x00],
            ),
        ];

        for (input, expected_output) in test_cases {
            let result = encode_varint(input).unwrap();
            assert_eq!(result, expected_output);
        }
    }
    #[test]
    fn varint_3() {
        print!("{:?}", hex(encode_varint(76u64).unwrap()));
    }
}

//4d4c4caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
