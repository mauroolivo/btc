use std::{fmt, io::{Cursor, Read}, vec};
use std::io::{Seek, SeekFrom};
use num::{BigUint, ToPrimitive, Zero};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use crate::tx_input::TxInput;
use crate::tx_output::TxOutput;
use crate::helpers::varint::{encode_varint, read_varint};
use crate::helpers::hash256::hash256;
use crate::helpers::sig_hash::SIGHASH_ALL;
use crate::private_key::PrivateKey;
use crate::script::Script;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Tx {
    version: u32,
    inputs: Vec<TxInput>,
    outputs: Vec<TxOutput>,
    locktime: u32,
    testnet: bool,
    segwit: bool,
    hash_prevouts: Option<Vec<u8>>,
    hash_sequence: Option<Vec<u8>>,
    hash_outputs: Option<Vec<u8>>,
}

impl Tx {
    pub fn new(version: u32, inputs: Vec<TxInput>, outputs: Vec<TxOutput>, locktime: u32, testnet: bool, segwit: bool) -> Self {
        Tx {
            version: version,
            inputs: inputs,
            outputs: outputs,
            locktime: locktime,
            testnet: testnet,
            segwit: segwit,
            hash_prevouts: None,
            hash_sequence: None,
            hash_outputs: None,
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
        let mut buffer = [0; 1];
        stream.read(&mut buffer)?;
        stream.seek(SeekFrom::Start(0))?;
        let mut is_segwit = false;
        if buffer[0] == 0x00 { // segwit marker
            is_segwit = true;
        }
        let mut buffer = [0; 4];
        stream.read(&mut buffer)?;
        let version = little_endian_to_int(buffer.as_slice()).to_u32().unwrap();
        if is_segwit {
            let mut buffer = [0; 2];
            stream.read(&mut buffer)?;
            if buffer != [0x00,0x01] { // segwit marker
                panic!("invalid segwit marker");
            }
        }
        let mut inputs: Vec<TxInput> = Vec::new();
        let mut outputs: Vec<TxOutput> = Vec::new();

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

        if is_segwit {
            for tx_in in inputs.iter_mut() {
                if let Ok(num_items) = read_varint(stream) {

                    let mut items: Vec<Vec<u8>> = vec![];
                    for _ in 0..num_items {
                        if let Ok(item_len) = read_varint(stream) {
                            if item_len == 0 {
                                items.push(vec![0])
                            } else {
                                let mut buffer: Vec<u8> = vec![0;item_len as usize];
                                stream.read(&mut buffer)?;
                                items.push(buffer)
                            }
                        }
                    }
                    tx_in.witness = Some(items);
                }
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
            segwit: is_segwit,
            hash_prevouts: None,
            hash_sequence: None,
            hash_outputs: None,
        })
    }
    pub fn serialize(&self) -> Vec<u8> {
        if self.segwit {
            self.serialize_segwit()
        } else {
            self.serialize_legacy()
        }
    }
    pub fn serialize_segwit(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend(int_to_little_endian(BigUint::from(self.version), 4));
        result.extend(vec![0x00, 0x01]);
        result.extend(encode_varint(self.inputs.len() as u64).unwrap());
        for tx_in in self.tx_ins() {
            result.extend(tx_in.serialize());
        }
        result.extend(encode_varint(self.outputs.len() as u64).unwrap());
        for tx_out in self.tx_outs() {
            result.extend(tx_out.serialize());
        }
        for tx_in in self.tx_ins() {
            match tx_in.witness {
                Some(witness) => {
                    result.extend(int_to_little_endian(BigUint::from(witness.len()), 1));
                    for item in witness {
                        if item.len() == 1 {
                            result.extend(int_to_little_endian(BigUint::from(item[0]), 1))
                        } else {
                            result.extend(encode_varint(item.len() as u64).unwrap());
                            result.extend(item);
                        }
                    }
                }
                None => {}
            }
        }
        result.extend(int_to_little_endian(BigUint::from(self.locktime), 4));
        result
    }
    pub fn serialize_legacy(&self) -> Vec<u8> {
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
    pub fn hash_prevouts(&mut self) -> Option<Vec<u8>> {
        let mut all_prevouts: Vec<u8> = vec![];
        let mut all_sequence: Vec<u8> = vec![];
        if self.hash_prevouts.is_none() {
            for tx_in in self.tx_ins() {
                let mut p_outs = tx_in.prev_tx().clone();
                p_outs.reverse();
                all_prevouts.extend(p_outs);
                all_prevouts.extend(int_to_little_endian(BigUint::from(tx_in.prev_index()), 4));
                all_sequence.extend(int_to_little_endian(BigUint::from(tx_in.sequence()), 4));
            }
            self.hash_prevouts = Some(hash256(all_prevouts.as_slice()).to_vec());
            self.hash_sequence = Some(hash256(all_sequence.as_slice()).to_vec());
        }
        self.hash_prevouts.clone()
    }



    pub fn hash_sequence(&mut self) -> Option<Vec<u8>> {
        if self.hash_prevouts.is_none() {
            self.hash_prevouts();
        }
        self.hash_sequence.clone()
    }
    pub fn hash_outputs(&mut self) -> Option<Vec<u8>> {
        let mut all_outputs: Vec<u8> = vec![];
        if self.hash_outputs.is_none() {
            for tx_out in self.tx_outs() {
                all_outputs.extend(tx_out.serialize());
            }
            self.hash_outputs = Some(hash256(all_outputs.as_slice()).to_vec());
        }
        self.hash_outputs.clone()
    }
    fn hash(&self) -> Vec<u8> {
        let bytes = self.serialize_legacy();
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
    pub fn sig_hash(&self, input_index: usize, redeem_script: Option<Script>) -> BigUint {
        let mut result = Vec::new();
        result.extend(int_to_little_endian(BigUint::from(self.version), 4));
        let num_ins = encode_varint(self.inputs.len() as u64).unwrap();
        result.extend(num_ins);

        for (idx, tx_in) in self.inputs.iter().enumerate() {
            if idx == input_index {
                // if the RedeemScript was passed in, that's the ScriptSig
                // otherwise the previous tx's ScriptPubkey is the ScriptSig
                match &redeem_script {
                    Some(script) => {
                        println!("WITH REDEEM");
                        let tx_input = TxInput::new(tx_in.prev_tx(), tx_in.prev_index(), script.clone(), tx_in.sequence());
                        result.extend(tx_input.serialize());
                    }

                    None => {
                        println!("NO REDEEM");
                        let tx_input = TxInput::new(tx_in.prev_tx(), tx_in.prev_index(), tx_in.script_pubkey(self.testnet), tx_in.sequence());
                        result.extend(tx_input.serialize());
                    }
                }
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
    pub fn sig_hash_bip143(&mut self, input_index: usize, redeem_script: Option<Script>, witness_script: Option<Script>) -> BigUint {

        let pr = self.hash_prevouts().unwrap();
        let se = self.hash_sequence().unwrap();

        let tx_in = &self.inputs[input_index];
        let mut s: Vec<u8> = Vec::new();
        // per BIP143 spec
        s.extend(int_to_little_endian(BigUint::from(self.version), 4));

        println!("pr: {}", hex::encode(pr.clone()));
        s.extend(pr);
        s.extend(se);

        let mut prev = tx_in.prev_tx();
        prev.reverse();
        s.extend(prev);
        s.extend(int_to_little_endian(BigUint::from(tx_in.prev_index()), 4));

        let mut script_code = Vec::new();
        if witness_script.is_some() {
                script_code = witness_script.unwrap().serialize()
        } else if redeem_script.is_some() {
            let script = redeem_script.unwrap();
            let h160 = script.cmds[1].clone();
            script_code = Script::p2pkh_script(h160).serialize();
        } else {
            let script = tx_in.script_pubkey(self.testnet);
            let h160 = script.cmds[1].clone();
            script_code = Script::p2pkh_script(h160).serialize();
        }
        s.extend(script_code.clone());
        s.extend(int_to_little_endian(BigUint::from(tx_in.value(self.testnet)), 8));
        s.extend(int_to_little_endian(BigUint::from(tx_in.sequence()), 4));
        s.extend(self.hash_outputs().unwrap());
        s.extend(int_to_little_endian(BigUint::from(self.locktime), 4));
        s.extend(int_to_little_endian(BigUint::from(SIGHASH_ALL), 4));

        println!("sc: {:?}", script_code);
        let hash = hash256(s.as_slice());
        BigUint::from_bytes_be(hash.as_slice())
    }

    pub fn verify_input(&mut self, input_index: usize) -> bool {
        let tx_ins = self.tx_ins(); //[input_index];
        let tx_in = &tx_ins[input_index];
        let prev_script_pubkey = tx_in.script_pubkey(self.testnet);

        let mut z: BigUint = BigUint::zero();
        let mut witness: Option<Vec<Vec<u8>>> = None;
        let mut redeem_script: Option<Script> = None;

        if prev_script_pubkey.is_p2sh_script_pubkey() {
            // the last cmd in a p2sh is the RedeemScript
            let mut script_sig = tx_in.script_sig.clone();
            let cmd = script_sig.cmds.pop().unwrap();
            let mut raw_redeem: Vec<u8> = vec![];
            let len_raw_redeem = encode_varint(cmd.len() as u64).unwrap();
            raw_redeem.extend(len_raw_redeem);
            raw_redeem.extend(cmd);
            let mut stream = Cursor::new(raw_redeem);
            match Script::parse(&mut stream) {
                Ok(script) => {
                    redeem_script = Some(script.clone());

                    if script.is_p2wpkh_script_pubkey() {
                        z = self.sig_hash_bip143(input_index, redeem_script.clone(), None);
                        witness = tx_in.witness.clone();
                    } else if redeem_script.clone().unwrap().is_p2wsh_script_pubkey() {
                        let mut raw_witness: Vec<u8> = vec![];
                        let mut part = tx_in.witness.clone().unwrap();
                        let cmd: Vec<u8> = part.pop().unwrap();
                        raw_witness.extend(encode_varint(cmd.len() as u64).unwrap());
                        raw_witness.extend(cmd);
                        let mut w_stream = Cursor::new(raw_witness);
                        let witness_script = Script::parse(&mut w_stream).unwrap();
                        z = self.sig_hash_bip143(input_index, None, Some(witness_script));
                        witness = tx_in.clone().witness;
                    } else {
                        z = self.sig_hash(input_index, redeem_script.clone());
                        witness = None;
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                    println!("{:?} {:?} {:?}", redeem_script, z, witness);
                    panic!("Can't parse redeem script");
                }
            }
        } else {

            if prev_script_pubkey.is_p2wpkh_script_pubkey() {

                z = self.sig_hash_bip143(input_index, None, None);
                witness = tx_in.clone().witness;

            } else if prev_script_pubkey.is_p2wsh_script_pubkey() {

                let mut raw_witness: Vec<u8> = Vec::new();
                let mut part = tx_in.witness.clone().unwrap();
                let cmd: Vec<u8> = part.pop().unwrap();
                raw_witness.extend(encode_varint(cmd.len() as u64).unwrap());
                raw_witness.extend(cmd);
                let mut w_stream = Cursor::new(raw_witness);
                let witness_script = Script::parse(&mut w_stream).unwrap();
                z = self.sig_hash_bip143(input_index, None, Some(witness_script));
                witness = tx_in.clone().witness;
            } else {
                z = self.sig_hash(input_index, None);
                witness = None;
            }
        }

        let ss = tx_in.script_sig();
        let pp = prev_script_pubkey;
        // println!("z: {}", z);
        // let w1 = &witness.clone().unwrap()[0];
        // let w2 = &witness.clone().unwrap()[1];
        // println!("witness 0 : {:?}", hex::encode(w1));
        // println!("witness 1 : {:?}", hex::encode(w2));
        // println!("ss: {}", ss.clone());
        // println!("pp: {}", pp.clone());

        let combined_script = ss + pp;
        combined_script.evaluate(&z.clone(), &witness.clone())
    }
    pub fn verify(&mut self) -> bool {
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
    pub fn sign_input(&mut self, input_index: usize, private_key: &PrivateKey) -> bool {
        let z = self.sig_hash(input_index, None);
        let der = private_key.sign(&z).der();
        let mut sig: Vec<u8> = vec![];
        sig.extend(der);
        sig.extend(SIGHASH_ALL.to_le_bytes());
        let sec = private_key.point().sec(false);
        let mut cmds: Vec<Vec<u8>> = vec![];
        cmds.push(sig);
        cmds.push(sec);
        let combined_script = Script::new(cmds);
        self.inputs[input_index].script_sig = combined_script;
        self.verify_input(input_index)
    }
    pub fn is_coinbase(&self) -> bool {
        if self.tx_ins().len() != 1 || self.tx_ins().len() == 0 {
            return false;
        }
        let first = &self.tx_ins()[0];
        if first.prev_tx() != [0u8;32] {
            return false;
        }
        if first.prev_index() != 0xffffffff {
            return false;
        }
        true
    }
    pub fn coinbase_height(&self) -> Option<BigUint> {
        if self.is_coinbase() {
            let first = &self.tx_ins()[0];
            let cmd = &first.script_sig.cmds[0];
            return Some(little_endian_to_int(cmd));
        }
        None
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
    use crate::private_key::PrivateKey;

    use super::*;
    #[test]
    fn test_parse_version() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, true).unwrap();
        assert_eq!(tx.version(), 1);
    }
    #[test]
    fn test_parse_inputs() {
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
    fn test_parse_locktime() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, true).unwrap();
        assert_eq!(tx.locktime, 410393u32);
    }
    #[test]
    fn test_parse_more() {
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
    fn test_parse_serialize() {
        let raw_tx = hex::decode("010000000456919960ac691763688d3d3bcea9ad6ecaf875df5339e148a1fc61c6ed7a069e010000006a47304402204585bcdef85e6b1c6af5c2669d4830ff86e42dd205c0e089bc2a821657e951c002201024a10366077f87d6bce1f7100ad8cfa8a064b39d4e8fe4ea13a7b71aa8180f012102f0da57e85eec2934a82a585ea337ce2f4998b50ae699dd79f5880e253dafafb7feffffffeb8f51f4038dc17e6313cf831d4f02281c2a468bde0fafd37f1bf882729e7fd3000000006a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937feffffff567bf40595119d1bb8a3037c356efd56170b64cbcc160fb028fa10704b45d775000000006a47304402204c7c7818424c7f7911da6cddc59655a70af1cb5eaf17c69dadbfc74ffa0b662f02207599e08bc8023693ad4e9527dc42c34210f7a7d1d1ddfc8492b654a11e7620a0012102158b46fbdff65d0172b7989aec8850aa0dae49abfb84c81ae6e5b251a58ace5cfeffffffd63a5e6c16e620f86f375925b21cabaf736c779f88fd04dcad51d26690f7f345010000006a47304402200633ea0d3314bea0d95b3cd8dadb2ef79ea8331ffe1e61f762c0f6daea0fabde022029f23b3e9c30f080446150b23852028751635dcee2be669c2a1686a4b5edf304012103ffd6f4a67e94aba353a00882e563ff2722eb4cff0ad6006e86ee20dfe7520d55feffffff0251430f00000000001976a914ab0c0b2e98b1ab6dbf67d4750b0a56244948a87988ac005a6202000000001976a9143c82d7df364eb6c75be8c80df2b3eda8db57397088ac46430600").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, true).unwrap();
        let ser = tx.serialize();
        assert_eq!(raw_tx, ser);
    }
    #[test]
    fn test_parse_serialize2() {
        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, false).unwrap();
        println!("{}", tx.id());
        let ser = tx.serialize();
        assert_eq!(raw_tx, ser);
    }

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
    #[test]
    fn test_sig_hash() {
        let z= BigUint::from_str_radix("27e0c5994dec7824e56dec6b2fcb342eb7cdb0d0957c2fce9882f715e85d81a6", 16).unwrap();
        let tx_id = "452c629d67e41baec3ac6f04fe744b4b9617f8f859c63b3002f8684e7a4fee03";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(tx) => {
                assert_eq!(tx.sig_hash(0, None), z);
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

        let tx = Tx::new(1u32, vec![tx_in], vec![change_output, target_output], 0u32, true, false);
        println!("{}", tx);
    }
    #[test]
    fn test_tx_sign() {

        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, false).unwrap();
        // tx sign
        let z = tx.sig_hash(0, None); // in this case we have only 1 input
        let hash = hash256(b"my secret");
        let e = BigUint::from_bytes_be(hash.as_slice());
        let private_key = PrivateKey::new(&e);
        let der = private_key.sign(&z).der();

        let mut sig: Vec<u8> = vec![];
        sig.extend(der);
        sig.extend(SIGHASH_ALL.to_le_bytes());
        let sec = private_key.point().sec(false);
        let mut cmds: Vec<Vec<u8>> = vec![];
        cmds.push(sig);
        cmds.push(sec);
        let script_sig = Script::new(cmds);

        let tx_in = tx.tx_ins()[0].clone();
        let tx_in_update = TxInput::new(tx_in.prev_tx(), tx_in.prev_index(), script_sig, tx_in.sequence());
        let tx = Tx::new(tx.version(), vec![tx_in_update], tx.tx_outs(), tx.locktime, tx.testnet, tx.segwit);
        println!("{}", tx);
        println!("{:?}", hex::encode(tx.serialize()));
    }
    #[test]
    fn test_verify_p2pkh() {

        let tx_id = "452c629d67e41baec3ac6f04fe744b4b9617f8f859c63b3002f8684e7a4fee03";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
        /* Testnet down
        let tx_id = "5418099cc755cb9dd3ebc6cf1a7888ad53a1a3beb5a025bce89eb1bf7f1650a2";
        let testnet = true;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
        */
    }
    #[test]
    fn test_verify_p2sh() {
        let tx_id = "46df1a9484d0a81d03ce0ee543ab6e1a23ed06175c104a178268fad381216c2b";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
    #[test]
    fn test_is_coinbase() {
        let raw_tx = hex::decode("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff5e03d71b07254d696e656420627920416e74506f6f6c20626a31312f4542312f4144362f43205914293101fabe6d6d678e2c8c34afc36896e7d9402824ed38e856676ee94bfdb0c6c4bcd8b2e5666a0400000000000000c7270000a5e00e00ffffffff01faf20b58000000001976a914338c84849423992471bffb1a54a8d9b1d69dc28a88ac00000000").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, false).unwrap();
        assert_eq!(tx.is_coinbase(), true);
    }
    #[test]
    fn genesis_script_sig() {
        let raw_script = hex::decode("4d04ffff001d0104455468652054696d65732030332f4a616e2f32303039204368616e63656c6c6f72206f6e206272696e6b206f66207365636f6e64206261696c6f757420666f722062616e6b73").unwrap();
        let mut stream = Cursor::new(raw_script);
        let s = Script::parse(&mut stream).unwrap();
        let cmds = s.cmds;
        println!("{}", std::str::from_utf8(&cmds[2]).unwrap());
    }
    #[test]
    fn test_coinbase_height() {
        let raw_tx = hex::decode("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff5e03d71b07254d696e656420627920416e74506f6f6c20626a31312f4542312f4144362f43205914293101fabe6d6d678e2c8c34afc36896e7d9402824ed38e856676ee94bfdb0c6c4bcd8b2e5666a0400000000000000c7270000a5e00e00ffffffff01faf20b58000000001976a914338c84849423992471bffb1a54a8d9b1d69dc28a88ac00000000").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, false).unwrap();
        assert_eq!(tx.coinbase_height().unwrap(), BigUint::from(465879u32));

        let raw_tx = hex::decode("0100000001813f79011acb80925dfe69b3def355fe914bd1d96a3f5f71bf8303c6a989c7d1000000006b483045022100ed81ff192e75a3fd2304004dcadb746fa5e24c5031ccfcf21320b0277457c98f02207a986d955c6e0cb35d446a89d3f56100f4d7f67801c31967743a9c8e10615bed01210349fc4e631e3624a545de3f89f5d8684c7b8138bd94bdd531d2e213bf016b278afeffffff02a135ef01000000001976a914bc3b654dca7e56b04dca18f2566cdaf02e8d9ada88ac99c39800000000001976a9141c4bc762dd5423e332166702cb75f40df79fea1288ac19430600").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, false).unwrap();
        assert!(tx.coinbase_height().is_none());
    }
    #[test]
    fn test_segwit_parse_1() {
        // tx_id 39cc1562b197182429bc1ea312c9e30f1257be6d5159fcd7b375139d3c3fe63c
        let raw_tx = hex::decode("020000000001011c20e4848e7992a8c23deff629105174d36286234429b4f6878a52a14c87931a0100000000fdffffff02cf21180000000000160014853ec3166860371ee67b7754ff85e13d7a0d669850330500000000001976a914fc71e34a661ea03b46b4e2414dac463d3328e12188ac02473044022007b6e8bb9f1cc0e3526ae158cfbd663debf56826249c3439f8967a0a7dd4244a022004dac7a6d79f37283ca739b2ec4ed502ec208eb05287fdc2a2a6df1ca83c10d0012103e5e444515d5566e7def1332d7dded8755ed9a2f1c8c968a3de1e72369a2ae7603d600a00").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, false).unwrap();
        println!("{:?}", tx);
    }
    #[test]
    fn test_segwit_parse_2() {
        // tx_id d869f854e1f8788bcff294cc83b280942a8c728de71eb709a2c29d10bfe21b7c
        let raw_tx = hex::decode("0100000000010115e180dc28a2327e687facc33f10f2a20da717e5548406f7ae8b4c811072f8560100000000ffffffff0100b4f505000000001976a9141d7cd6c75c2e86f4cbf98eaed221b30bd9a0b92888ac02483045022100df7b7e5cda14ddf91290e02ea10786e03eb11ee36ec02dd862fe9a326bbcb7fd02203f5b4496b667e6e281cc654a2da9e4f08660c620a1051337fa8965f727eb19190121038262a6c6cec93c2d3ecd6c6072efea86d02ff8e3328bbd0242b20af3425990ac00000000").unwrap();
        let mut stream = Cursor::new(raw_tx);
        let tx = Tx::parse(&mut stream, true).unwrap();
        println!("{:?}", tx);
    }
    #[test]
    fn test_segwit_serialize_1() {
        let raw_tx = hex::decode("020000000001011c20e4848e7992a8c23deff629105174d36286234429b4f6878a52a14c87931a0100000000fdffffff02cf21180000000000160014853ec3166860371ee67b7754ff85e13d7a0d669850330500000000001976a914fc71e34a661ea03b46b4e2414dac463d3328e12188ac02473044022007b6e8bb9f1cc0e3526ae158cfbd663debf56826249c3439f8967a0a7dd4244a022004dac7a6d79f37283ca739b2ec4ed502ec208eb05287fdc2a2a6df1ca83c10d0012103e5e444515d5566e7def1332d7dded8755ed9a2f1c8c968a3de1e72369a2ae7603d600a00").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, true).unwrap();
        let ser = tx.serialize();
        assert_eq!(raw_tx, ser);
    }
    #[test]
    fn test_segwit_serialize_2() {
        let raw_tx = hex::decode("0100000000010115e180dc28a2327e687facc33f10f2a20da717e5548406f7ae8b4c811072f8560100000000ffffffff0100b4f505000000001976a9141d7cd6c75c2e86f4cbf98eaed221b30bd9a0b92888ac02483045022100df7b7e5cda14ddf91290e02ea10786e03eb11ee36ec02dd862fe9a326bbcb7fd02203f5b4496b667e6e281cc654a2da9e4f08660c620a1051337fa8965f727eb19190121038262a6c6cec93c2d3ecd6c6072efea86d02ff8e3328bbd0242b20af3425990ac00000000").unwrap();
        let mut stream = Cursor::new(raw_tx.clone());
        let tx = Tx::parse(&mut stream, false).unwrap();
        println!("{}", tx.id());
        let ser = tx.serialize();
        assert_eq!(raw_tx, ser);
    }
    #[ignore]
    #[test]
    fn test_verify_p2wpkh() {
        let tx_id = "d869f854e1f8788bcff294cc83b280942a8c728de71eb709a2c29d10bfe21b7c";
        let testnet = true;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_verify_p2sh_p2wpkh() {
        let tx_id = "c586389e5e4b3acb9d6c8be1c19ae8ab2795397633176f5a6442a261bbdefc3a";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[ignore]
    #[test]
    fn test_verify_p2wsh() {
        let tx_id = "78457666f82c28aa37b74b506745a7c7684dc7842a52a457b09f09446721e11c";
        let testnet = true;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[ignore]
    #[test]
    fn test_verify_p2sh_p2wsh() {
        let tx_id = "954f43dbb30ad8024981c07d1f5eb6c9fd461e2cf1760dd1283f052af746fc88";
        let testnet = true;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[ignore]
    #[test]
    fn test_verify_more_1() {
        let tx_id = "b28af11d837f5451a480d8f116c107bcd3c6d087927bcbb49ff01307a57fd483";
        let testnet = true;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_verify_more_2() {
        let tx_id = "e12d37756420b2333e37a7d19479e859d43340c19b7f7391af9d360417aa0341";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_verify_more_3() {
        let tx_id = "d12973665f0a5cd7d493873ce10e0bad3b04361dc723ed011e314d0b4877a814";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_verify_more_4() {
        let tx_id = "64ff0b827f7899674fc26b693c557852540b9260c5c29cf18f536b56f01b17ba";
        let testnet = false;
        let tf = TxFetcher::new(testnet);
        let result = tf.fetch_sync(tx_id);
        match result {
            Ok(mut tx) => {
                println!("{:?}", tx);
                assert_eq!(tx.verify(), true);
            }
            Err(e) => {
                println!("{:?}", e);
                assert!(false);
            }
        }
    }
}
