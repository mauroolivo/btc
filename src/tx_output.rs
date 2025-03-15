
use crate::script::Script;
use std::{fmt, io::{Cursor, Read}};
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TxOutput {
    amount: u64,
    script_pubkey: Script,
}
impl TxOutput {
    pub fn new(amount: u64, script_pubkey: Script) -> TxOutput {
        TxOutput {
            amount,
            script_pubkey,
        }
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, std::io::Error> {
        let mut buffer = [0; 8];
        stream.read(&mut buffer)?;
        let script_pubkey = Script::parse(stream)?;
        Ok(TxOutput {
            amount: little_endian_to_int(buffer.as_slice()).to_u64().unwrap(),
            script_pubkey: script_pubkey,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend( int_to_little_endian(BigUint::from(self.amount), 8u32));
        result.extend(self.script_pubkey.serialize());
        result
    }
    pub fn amount(&self) -> u64 {
        self.amount
    }
    pub fn script_pubkey(&self) -> Script {
        self.script_pubkey.clone()
    }
}
impl fmt::Display for TxOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TxOutput {{ value: {}, script_pubkey: {} }}",
            self.amount(),
            self.script_pubkey()
        )
    }
}