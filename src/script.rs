use std::{io::{Cursor, Read, Error}};
use crate::helpers::varint::{encode_varint, read_varint};
use core::fmt;
use num::ToPrimitive;
use crate::helpers::endianness::{little_endian_to_int};
use crate::helpers::op::op_code_names;
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Script {
    cmds: Vec<Vec<u8>>,
}
impl Script {
    pub fn new(cmds: Vec<Vec<u8>>) -> Self {
        Self { cmds: cmds }
    }
}
impl Script {
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Script, Error> {
        let mut cmds = vec![];
        let mut count = 0;
        let length = read_varint(stream)?; // length of entire script
        while count < length {
            let mut current = [0u8; 1];
            stream.read(&mut current)?;
            count += 1;
            let current_byte = current[0];
            print!("{}", current_byte);
            if current_byte >= 1 && current_byte <= 75 { // read n bytes as an element
                let n = current_byte;
                let mut cmd = vec![0u8; n as usize];
                stream.read(&mut cmd)?;
                cmds.push(cmd);
                count += n as u64;
            } else if current_byte == 76 { // 76 OP_PUSHDATA1
                let mut buffer = [0; 1];
                stream.read(&mut buffer)?;
                let ln = little_endian_to_int(buffer.as_slice()).to_u16().unwrap();
                let mut cmd = vec![0; ln.to_usize().unwrap()];
                stream.read(&mut cmd)?;
                cmds.push(cmd);
                count += ln as u64 + 1;
            } else if current_byte == 77 { // 76 OP_PUSHDATA2
                let mut buffer = [0; 2];
                stream.read(&mut buffer)?;
                let ln = little_endian_to_int(buffer.as_slice()).to_u16().unwrap();
                let mut cmd = vec![0; ln.to_usize().unwrap()];
                stream.read(&mut cmd)?;
                cmds.push(cmd);
                count += ln as u64 + 2;
            } else {
                let op_code = current_byte;
                cmds.push(vec![op_code]);
            }
        }
        if count != length {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "parsing script failed",
            ));
        }
        Ok(Script { cmds })
    }
    fn raw_serialize(&self) -> Vec<u8> {
        let mut result = vec![];
        for cmd in &self.cmds {
            if cmd.len() == 1 {
                let op_code = cmd[0];
                result.push(op_code);
            } else {
                let length = cmd.len();
                if length < 76 {
                    result.push(length as u8);
                } else if length <= 0xff {
                    result.push(76);
                    result.push(length as u8);
                } else if length <= 520 {
                    result.push(77);
                    result.extend_from_slice(&length.to_le_bytes()[..2]);
                } else {
                    panic!("too long a cmd");
                }
                result.extend_from_slice(&cmd);
            }
        }
        result
    }
    pub fn serialize(&self) -> Vec<u8> {
        let raw_result = self.raw_serialize();
        let len = raw_result.len();
        let mut result = vec![];
        let len_encoded = encode_varint(len as u64).unwrap();
        result.extend(len_encoded);
        result.extend(raw_result);
        result
    }
}
impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_code_names = op_code_names();
        let mut result = String::new();

        for cmd in &self.cmds {
            if cmd.len() == 1 {
                let op_code = cmd[0];
                result.push_str(&op_code_names[&op_code]);
            } else {
                result.push_str(
                    &cmd.iter()
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<String>(),
                );
            }
            result.push(' ');
        }

        write!(f, "{}", result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {

        let script_pubkey = hex::decode("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937").unwrap();
        let mut stream = Cursor::new(script_pubkey.clone());
        let script = Script::parse(stream.by_ref()).unwrap();
        println!("{}", script);
        let required = hex::decode("304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601").unwrap();
        assert_eq!(script.cmds[0], required);
        let required = hex::decode("035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937").unwrap();
        assert_eq!(script.cmds[1], required);

        // fake test OP_PUSHDATA2
        let script_pubkey = hex::decode("FD03014d0001aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let mut stream = Cursor::new(script_pubkey.clone());
        let script = Script::parse(stream.by_ref()).unwrap();
        println!("{}", script);
        let required = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        assert_eq!(script.cmds[0], required);

        // fake test OP_PUSHDATA1
        let script_pubkey = hex::decode("4F4c4caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let mut stream = Cursor::new(script_pubkey.clone());
        let script = Script::parse(stream.by_ref()).unwrap();
        println!("{}", script);
        let required = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        assert_eq!(script.cmds[0], required);
    }
    #[test]
    fn test_serialize() {
        let want = "6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937";
        let script_pubkey = hex::decode(want).unwrap();
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        println!("{}", script);
        assert_eq!(hex::encode(script.serialize()), want);

        let full = "fd03014d0001aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let script_pubkey = hex::decode(full).unwrap();
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        println!("{}", script);
        assert_eq!(hex::encode(script.serialize()), full);

        let full = "4e4c4caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let script_pubkey = hex::decode(full).unwrap();
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        println!("{}", script);
        assert_eq!(hex::encode(script.serialize()), full);
    }
}