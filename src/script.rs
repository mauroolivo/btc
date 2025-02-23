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
}