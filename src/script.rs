use std::{io::{Cursor, Read, Error}};
use crate::helpers::varint::read_varint;

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
        let length = read_varint(stream)?;
        while count < length {
            let mut current = [0u8; 1];
            stream.read(&mut current)?;
            count += 1;
            let current_byte = current[0];
            if current_byte >= 1 && current_byte <= 75 {
                let n = current_byte;
                let mut cmd = vec![0u8; n as usize];
                stream.read(&mut cmd)?;
                cmds.push(cmd);
                count += n as u64;
            } else if current_byte == 76 {
                let data_length = read_varint(stream)?;
                let mut cmd = vec![0u8; data_length as usize];
                stream.read(&mut cmd)?;
                cmds.push(cmd);
                count += data_length as u64 + 1;
            } else if current_byte == 77 {
                let data_length = read_varint(stream)?;
                let mut cmd = vec![0u8; data_length as usize];
                stream.read(&mut cmd)?;
                cmds.push(cmd);
                count += data_length as u64 + 2;
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
        let mut result = self.raw_serialize();
        let total = result.len();
        let mut length_bytes = vec![];
        if total < 0xfd {
            length_bytes.push(total as u8);
        } else if total <= 0xffff {
            length_bytes.push(0xfd);
            length_bytes.extend_from_slice(&(total as u16).to_le_bytes());
        } else {
            length_bytes.push(0xfe);
            length_bytes.extend_from_slice(&(total as u32).to_le_bytes());
        }
        length_bytes.append(&mut result);
        length_bytes
    }
}