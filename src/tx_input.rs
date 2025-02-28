use crate::script::Script;
use std::{io::{Cursor, Read, Error}};
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use crate::tx_fetcher::TxFetcher;
use crate::helpers::hex::hex;
use crate::tx::Tx;

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

    pub fn fetch_tx(&self, testnet: bool) -> Result<Tx, reqwest::Error> {
        let tx_id = hex(self.prev_tx().to_vec());
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id.as_str());
        match result {
            Ok(tx) => Ok(tx),
            Err(e) => Err(e)
        }
    }
    pub fn value(&self, testnet: bool) -> u64 {
        let tx = self.fetch_tx(testnet).unwrap();
        tx.tx_outs()[self.prev_index as usize].amount()
    }
    /*
    def fetch_tx(self, testnet=False):
    return TxFetcher.fetch(self.prev_tx.hex(), testnet=testnet)

    def value(self, testnet=False):
    '''Get the output value by looking up the tx hash.
    Returns the amount in satoshi.
    '''
    tx = self.fetch_tx(testnet=testnet)
    return tx.tx_outs[self.prev_index].amount
    */

}