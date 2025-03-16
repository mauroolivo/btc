use std::{fmt, io::{Cursor, Read}};
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;
use crate::helpers::varint::{encode_varint, read_varint};
use crate::helpers::hash256::hash256;
use crate::helpers::sig_hash::SIGHASH_ALL;
use crate::script::Script;

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
        for tx_out in self.tx_outs() {
            result.extend(tx_out.serialize());
        }
        result.extend(int_to_little_endian(BigUint::from(self.locktime), 4));
        result
    }
    pub fn id(&self) -> String {
        hex::encode(self.hash())
    }
    fn hash(&self) -> Vec<u8> {
        let bytes = self.serialize();
        let mut hash = hash256(&bytes);
        hash.reverse();
        hash.to_vec()
    }
    pub fn fee(&self) -> i64 {
        let mut sum_tx_ins: u64 = 0;
        let mut sum_tx_outs: u64 = 0;
        for tx_in in self.tx_ins() {
            sum_tx_ins += tx_in.value(self.testnet)
        }
        for tx_out in self.tx_outs() {
            sum_tx_outs += tx_out.amount()
        }
        sum_tx_ins as i64 - sum_tx_outs as i64
    }
    pub fn sig_hash(&self, input_index: usize) -> BigUint {

        let mut result = Vec::new();
        result.extend(int_to_little_endian(BigUint::from(self.version), 4));
        let num_ins = encode_varint(self.inputs.len() as u64).unwrap();

        result.extend(num_ins);
        for (idx, tx_in) in self.inputs.iter().enumerate() {
            if idx == input_index {
                let tx_input = TxInput::new(tx_in.prev_tx(), tx_in.prev_index(), tx_in.script_pubkey(self.testnet), tx_in.sequence());
                result.extend(tx_input.serialize());
            } else {
                let tx_input = TxInput::new(tx_in.prev_tx(), tx_in.prev_index(), Script::new(vec![]), tx_in.sequence());
                result.extend(tx_input.serialize());
            }
        }
        result.extend(encode_varint(self.outputs.len() as u64).unwrap());
        for tx_out in self.tx_outs() {
            result.extend(tx_out.serialize());
        }
        result.extend(int_to_little_endian(BigUint::from(self.locktime), 4));
        result.extend(int_to_little_endian(BigUint::from(SIGHASH_ALL), 4));
        let hash = hash256(&result);
        let z = BigUint::from_bytes_be(hash.as_slice());
        z
    }
    pub fn verify_input(&self, input_index: usize) -> bool {
        let tx_ins = self.tx_ins(); //[input_index];
        let tx_in = &tx_ins[input_index];
        let prev_script_pubkey = tx_in.script_pubkey(self.testnet);
        let z = self.sig_hash(input_index);
        let combined_script = tx_in.script_sig() + prev_script_pubkey;
        combined_script.evaluate(&z)
    }
    pub fn verify(&self) -> bool {

        if self.fee() < 0 {
            println!("----------> fee is negative");
            return false;
        }

        for i in 0..self.tx_ins().len() {
            if !self.verify_input(i) {
                println!("----------> input is invalid {}/{}", i, self.tx_ins().len());
                return false;
            }
        }
        true
    }
}

impl fmt::Display for Tx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut inputs_string = String::new();
        for i in &self.inputs {
            inputs_string = format!("{}", i);
        }
        let mut outputs_string = String::new();
        for o in self.tx_outs() {
            outputs_string.push_str(format!("{} ", o).as_str());
        }
        write!(
            f,
            "id: {}, version: {}, inputs: {}, outputs: {}, locktime: {}",
            self.id(),
            self.version,
            inputs_string,
            outputs_string,
            self.locktime
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::tx_fetcher::TxFetcher;
    use num::Num;
    use crate::helpers::base58::decode_base58;
    use crate::script::Script;
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
    #[test]
    fn parse_serialize2() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, false).unwrap();
        println!("{}", tx.id());
        let ser = tx.serialize();
        assert_eq!(raw_tx, ser);
    }
    #[ignore]
    #[test]
    fn test_fee() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, false).unwrap();
        assert_eq!(tx.fee(), 40000);
        let raw_tx = hex::decode("010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, false).unwrap();
        assert_eq!(tx.fee(), 140500);
    }
    #[ignore]
    #[test]
    fn test_sig_hash() {
        let z= BigUint::from_str_radix("27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6", 16).unwrap();
        let tx_id = "452c629d67e41baec3ac6f04fe744b4b9617f8f859c63b3002f8684e7a4fee03";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(tx) => {
                assert_eq!(tx.sig_hash(0), z);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_tx_create() {
        // tx create
        let prev_tx = hex::decode("0d6fe5213c0b3291f208cba8bfb59b7476dffacc4e5cb66f6eb20a080843a299").unwrap();
        let prev_tx_index = 13u32;
        let sequence = 0xffffffffu32;
        let tx_in = TxInput::new(prev_tx, prev_tx_index, Script::new(vec![]), sequence);

        let satoshi = 100_000_000u64;
        // target
        let target_amount: u64 = (0.1f64 * satoshi as f64) as u64;
        let target_h160 = decode_base58("mnrVtF8DWjMu839VW3rBfgYaAfKk8983Xf".as_bytes().to_vec());
        let target_script = Script::p2pkh_script(target_h160);
        let target_output = TxOutput::new(target_amount, target_script);
        // change
        let change_amount: u64 = (0.33f64 * satoshi as f64) as u64;
        let change_h160 = decode_base58("mzx5YhAH9kNHtcN481u6WkjeHjYtVeKVh2".as_bytes().to_vec());
        let change_script = Script::p2pkh_script(change_h160);
        let change_output = TxOutput::new(change_amount, change_script);

        let tx = Tx::new(1u32, vec![tx_in], vec![change_output, target_output], 0u32, true);
        println!("{}", tx);
    }
    #[ignore]
    #[test]
    fn test_verify_p2pkh() {

        let tx_id = "452c629d67e41baec3ac6f04fe744b4b9617f8f859c63b3002f8684e7a4fee03";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(_) => {
                assert!(false);
            }
        }

        let tx_id = "5418099cc755cb9dd3ebc6cf1a7888ad53a1a3beb5a025bce89eb1bf7f1650a2";
        let testnet = true;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
    /* to be implemented for p2sh
    #[test]
    fn test_verify_p2sh() {
        let tx_id = "46df1a9484d0a81d03ce0ee543ab6e1a23ed06175c104a178268fad381216c2b";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
     */
}
