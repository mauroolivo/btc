use std::{fmt, io::{Cursor, Read}};
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;
use crate::helpers::varint::{encode_varint, read_varint};

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
    pub fn tx_ins(&self) -> Vec<TxInput> {
        self.inputs.clone()
    }
    pub fn tx_outs(&self) -> Vec<TxOutput> {
        self.outputs.clone()
    }
    pub fn locktime(&self) -> u32 {
        self.locktime
    }
    pub fn parse(stream: &mut Cursor<Vec<u8>>, testnet: bool) -> Result<Self, std::io::Error> {
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let version = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();

        let mut inputs:Vec<TxInput> = Vec::new();
        let mut outputs:Vec<TxOutput> = Vec::new();

        if let Ok(num_inputs) = read_varint(stream) {
            for _ in 0..num_inputs {
                inputs.push(TxInput::parse(stream).unwrap());
            }
        }
        //let mut outputs = vec![];
        if let Ok(num_outputs) = read_varint(stream) {
            for _ in 0..num_outputs {
                outputs.push(TxOutput::parse(stream).unwrap());
            }
        }

        let mut buffer = vec![0; 4];
        stream.read(&mut buffer).unwrap();
        let locktime = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();

        Ok(Tx {
            version,
            inputs,
            outputs,
            locktime,
            testnet,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(int_to_little_endian(BigUint::from(self.version), 4));
        result.extend(encode_varint(self.inputs.len() as u64).unwrap());
        for tx_in in self.tx_ins() {
            result.extend(tx_in.serialize());
        }
        result.extend(encode_varint(self.outputs.len() as u64).unwrap());
        for tx_in in self.tx_outs() {
            result.extend(tx_in.serialize());
        }
        result.extend(int_to_little_endian(BigUint::from(self.locktime), 4));
        result
    }
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
        let tx = Tx::parse(&mut stream, true).unwrap();
        assert_eq!(tx.version(), 1);
    }
    #[test]
    fn parse_inputs() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, true).unwrap();
        let requested = hex::decode("d1c789a9c60383bf715f3f6ad9d14b91fe55f3deb369fe5d9280cb1a01793f81").unwrap();
        let inputs: Vec<TxInput> = tx.tx_ins();

        assert_eq!(inputs.first().unwrap().prev_tx(), requested);
        assert_eq!(inputs.first().unwrap().prev_index(), 0u32);

        //let requested = hex::decode("6b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278a").unwrap();
        // assert_eq!(inputs.first().get_script_sig().serialize(), requested);
        assert_eq!(inputs.first().unwrap().sequence(), 0xfffffffe);
    }
    #[test]
    fn parse_outputs() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, true).unwrap();
        assert_eq!(tx.tx_outs().len(), 2);

        let requested = 32454049u64;
        assert_eq!(tx.tx_outs()[0].amount(), requested);

        // let requested = hex:: decode("1976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac").unwrap();
        // assert_eq!(tx.tx_outs()[0].script_pubkey().serialize(), requested);

        let requested = 10011545u64;
        assert_eq!(tx.tx_outs()[1].amount(), requested);

        // let required = hex:: decode("1976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac").unwrap();
        // assert_eq!(tx.tx_outs()[1].script_pubkey().serialize(), required);
    }
    #[test]
    fn parse_locktime() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, true).unwrap();
        assert_eq!(tx.locktime, 410393u32);
    }
    #[test]
    fn parse_more() {
        let raw_tx = hex::decode("010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, true).unwrap();

        //304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601 035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937
        // let ss1 = tx.tx_ins()[1].script_sig();

        //OP_DUP OP_HASH160 ab0c0b2e98b1ab6dbf67d4750b0a56244948a879 OP_EQUALVERIFY OP_CHECKSIG
        //let sp0 = tx.tx_outs()[0].script_pubkey();


        let requested = 40000000u64;
        assert_eq!(tx.tx_outs()[1].amount(), requested);

    }
    #[test]
    fn parse_serialize() {
        let raw_tx = hex::decode("010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, true).unwrap();
        let ser = tx.serialize();
        assert_eq!(raw_tx, ser);
    }
}