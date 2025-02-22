use std::{fmt, io::{Cursor, Read}};
use num::ToPrimitive;
use crate::helpers::endianness::little_endian_to_int;
use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tx {
    version: u32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: u32,
    testnet: bool,
}

impl Tx {
    pub fn new(version: u32, inputs: Vec<TxInput>, outputs: Vec<TxOutput>, locktime: u32, testnet: bool) -> Self {
        Tx {
            version: version,
            inputs: inputs,
            outputs: outputs,
            locktime: locktime,
            testnet: testnet,
        }
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>) -> Result<Self, std::io::Error> {
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let version = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();


        let inputs = Vec::new();
        let outputs = Vec::new();
        let locktime = 0;
        let testnet = true;
        /*        let mut inputs = vec![];
                if let Ok(num_inputs) = read_varint(stream) {
                    for _ in 0..num_inputs {
                        inputs.push(TxInput::parse(stream).unwrap());
                    }
                }

                let mut outputs = vec![];
                if let Ok(num_outputs) = read_varint(stream) {
                    for _ in 0..num_outputs {
                        outputs.push(TxOutput::parse(stream).unwrap());
                    }
                }

                let mut locktime = vec![0; 4];
                stream.read_exact(&mut locktime).unwrap();
                let locktime = u32::from_le_bytes(locktime.try_into().unwrap());*/

        Ok(Tx {
            version,
            inputs,
            outputs,
            locktime,
            testnet,
        })
        // pub fn id(&self) -> String {
        //     hex::encode(self.hash())
        // }

        // fn hash(&self) -> Vec<u8> {
        //     let bytes = self.serialize();
        //     let mut hash = hash256(&bytes);
        //     hash.reverse();
        //     hash.to_vec()
        // }
    }
}

impl fmt::Display for Tx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "version: {}, inputs: {}, outputs: {}, locktime: {}",
            self.version,
            self.inputs.len(),
            self.outputs.len(),
            self.locktime
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_version() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream).unwrap();
        assert_eq!(tx.version(), 1);
    }
}