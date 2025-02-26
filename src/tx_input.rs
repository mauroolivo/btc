use crate::script::Script;
use std::{io::{Cursor, Read, Error}};
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TxInput {
    prev_tx: Vec<u8>,
    prev_index: u32,
    script_sig: Script,
    sequence: u32,
}
impl TxInput {
    //pub fn new(prev_tx: Vec<u8>, prev_index: Vec<u8>, script_sig: Script, sequence: Vec<u8>) -> Self {}
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, Error> {
        let mut buffer = vec![0; 32];
        stream.read(&mut buffer)?;
        buffer.reverse();
        let prev_tx = buffer.clone();

        let mut buffer = vec![0; 4];
        stream.read(&mut buffer)?;
        let prev_index = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();

        let script_sig = Script::parse(stream)?;

        let mut buffer = vec![0; 4];
        stream.read(&mut buffer)?;
        let sequence = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();

        Ok(TxInput {
            prev_tx,
            prev_index,
            script_sig,
            sequence,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();

        let mut prev_tx = self.prev_tx.clone();
        prev_tx.reverse();
        result.extend(&prev_tx);
        result.extend(int_to_little_endian(BigUint::from(self.prev_index), 4u32));
        result.extend(self.script_sig.serialize());
        result.extend(int_to_little_endian(BigUint::from(self.sequence), 4u32));
        result
    }
    pub fn prev_tx(&self) -> &[u8] {
        &self.prev_tx
    }
    pub fn prev_index(&self) -> u32 {
        self.prev_index
    }
    pub fn sequence(&self) -> u32 {
        self.sequence
    }
    pub fn script_sig(&self) -> Script {
        self.script_sig.clone()
    }
}