
use crate::script::Script;
use std::{io::{Cursor, Read}};
use num::ToPrimitive;
use crate::helpers::endianness::little_endian_to_int;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TxOutput {
    amount: u64,
    script_pubkey: Script,
}
impl TxOutput {
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, std::io::Error> {
        let mut buffer = [0; 8];
        stream.read(&mut buffer)?;
        let script_pubkey = Script::parse(stream)?;
        Ok(TxOutput {
            amount: little_endian_to_int(buffer.as_slice()).to_u64().unwrap(),
            script_pubkey: script_pubkey,
        })
    }
    pub fn amount(&self) -> u64 {
        self.amount
    }
    pub fn script_pubkey(&self) -> Script {
        self.script_pubkey.clone()
    }
}