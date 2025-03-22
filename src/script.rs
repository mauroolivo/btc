use std::ops::{Add};
use std::{io::{Cursor, Read, Error}};
use crate::helpers::varint::{encode_varint, read_varint};
use core::fmt;
use num::{BigUint, ToPrimitive};
use crate::helpers::endianness::{int_to_little_endian, little_endian_to_int};
use crate::helpers::op_codes::*;

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
        let length = read_varint(stream)?; // length of entire script
        while count < length {
            let mut current = [0u8; 1];
            stream.read(&mut current)?;
            count += 1;
            let current_byte = current[0];

            match current_byte {
                _len @ 1..=75 => {
                    let n = current_byte;
                    let mut cmd = vec![0u8; n as usize];
                    stream.read(&mut cmd)?;
                    cmds.push(cmd);
                    count += n as u64;
                }
                OP_PUSHDATA1 => {
                    let mut buffer = [0; 1];
                    stream.read(&mut buffer)?;
                    let ln = little_endian_to_int(buffer.as_slice()).to_u16().unwrap();
                    let mut cmd = vec![0; ln.to_usize().unwrap()];
                    stream.read(&mut cmd)?;
                    cmds.push(cmd);
                    count += ln as u64 + 1;
                }
                OP_PUSHDATA2 => {
                    let mut buffer = [0; 2];
                    stream.read(&mut buffer)?;
                    let ln = little_endian_to_int(buffer.as_slice()).to_u16().unwrap();
                    let mut cmd = vec![0; ln.to_usize().unwrap()];
                    stream.read(&mut cmd)?;
                    cmds.push(cmd);
                    count += ln as u64 + 2;
                }
                _ => {
                    let op_code = current_byte;
                    cmds.push(vec![op_code]);
                }
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
                if !is_op(&cmd) {
                    panic!("OP no handled");
                }
                let op_code = cmd[0];
                result.extend(int_to_little_endian(BigUint::from(op_code), 1));
            } else {
                let length = cmd.len();
                if length < 75 {
                    result.extend(int_to_little_endian(BigUint::from(length.to_u64().unwrap()), 1));
                } else if length < 256 {
                    result.extend(int_to_little_endian(BigUint::from(76u64), 1));
                    result.extend(int_to_little_endian(BigUint::from(length), 1));
                } else if length < 520 {
                    result.extend(int_to_little_endian(BigUint::from(77u64), 1));
                    result.extend(int_to_little_endian(BigUint::from(length), 2));
                } else {
                    panic!("too long a cmd");
                }
                result.extend(cmd)
            }
        }
        result
    }
    pub fn serialize(&self) -> Vec<u8> {
        let raw_result = self.raw_serialize();
        let len = raw_result.len();
        let mut result = vec![];
        let len_encoded = encode_varint(len as u64).unwrap();
        result.extend(len_encoded);
        result.extend(raw_result);
        result
    }
    pub fn evaluate(&self, z: &BigUint) -> bool {
        let mut cmds = self.cmds.clone();
        let mut stack: Vec<Vec<u8>> = vec![];
        let mut altstack: Vec<Vec<u8>> = vec![];
        while cmds.len() > 0 {

            let cmd = cmds.remove(0);
            if cmd.len() == 1 {
                if !is_op(&cmd) {
                    panic!("OP not handled {:?}", cmd);
                }
                let op_code = cmd[0];
                match op_code {
                    OP_0 => { if !op_0(&mut stack) { return false; } }
                    OP_1NEGATE => { if !op_1negate(&mut stack) { return false; } }

                    OP_1 => { if !op_1(&mut stack) { return false; } }
                    OP_2 => { if !op_2(&mut stack) { return false; } }
                    OP_3 => { if !op_3(&mut stack) { return false; } }
                    OP_4 => { if !op_4(&mut stack) { return false; } }
                    OP_5 => { if !op_5(&mut stack) { return false; } }
                    OP_6 => { if !op_6(&mut stack) { return false; } }
                    OP_7 => { if !op_7(&mut stack) { return false; } }
                    OP_8 => { if !op_8(&mut stack) { return false; } }
                    OP_9 => { if !op_9(&mut stack) { return false; } }
                    OP_10 => { if !op_10(&mut stack) { return false; } }
                    OP_11 => { if !op_11(&mut stack) { return false; } }
                    OP_12 => { if !op_12(&mut stack) { return false; } }
                    OP_13 => { if !op_13(&mut stack) { return false; } }
                    OP_14 => { if !op_14(&mut stack) { return false; } }
                    OP_15 => { if !op_15(&mut stack) { return false; } }
                    OP_16 => { if !op_16(&mut stack) { return false; } }
                    OP_NOP => { if !op_nop(&mut stack) { return false; } }
                    // OP_IF => {}
                    // OP_NOTIF => {}
                    // OP_ELSE => {}
                    // OP_ENDIF => {}
                    OP_VERIFY => { if !op_verify(&mut stack) { return false; } }
                    OP_RETURN => { if !op_return(&mut stack) { return false; } }
                    OP_TOALTSTACK => { if !op_toaltstack(&mut stack, &mut altstack) { return false; } }
                    OP_FROMALTSTACK => { if !op_fromaltstack(&mut stack, &mut altstack) { return false; } }
                    OP_2DROP => { if !op_2drop(&mut stack) { return false; } }
                    OP_2DUP => { if !op_2dup(&mut stack) { return false; } }
                    OP_3DUP => { if !op_3dup(&mut stack) { return false; } }
                    OP_2OVER => { if !op_2over(&mut stack) { return false; } }
                    OP_2ROT => { if !op_2rot(&mut stack) { return false; } }
                    OP_2SWAP => { if !op_2swap(&mut stack) { return false; } }
                    OP_IFDUP => { if !op_ifdup(&mut stack) { return false; } }
                    OP_DEPTH => { if !op_depth(&mut stack) { return false; } }
                    OP_DROP => { if !op_drop(&mut stack) { return false; } }
                    OP_DUP => { if !op_dup(&mut stack) { return false; } }
                    OP_NIP => { if !op_nip(&mut stack) { return false; } }
                    OP_OVER => { if !op_over(&mut stack) { return false; } }
                    OP_PICK => { if !op_pick(&mut stack) { return false; } }
                    OP_ROLL => { if !op_roll(&mut stack) { return false; } }
                    OP_ROT => { if !op_rot(&mut stack) { return false; } }
                    OP_SWAP => { if !op_swap(&mut stack) { return false; } }
                    OP_TUCK => { if !op_tuck(&mut stack) { return false; } }
                    OP_SIZE => { if !op_size(&mut stack) { return false; } }
                    OP_EQUAL => { if !op_equal(&mut stack) { return false; } }
                    OP_EQUALVERIFY => { if !op_equalverify(&mut stack) { return false; } }
                    OP_1ADD => { if !op_1add(&mut stack) { return false; } }
                    OP_1SUB => { if !op_1sub(&mut stack) { return false; } }
                    OP_NEGATE => { if !op_negate(&mut stack) { return false; } }
                    OP_ABS => { if !op_abs(&mut stack) { return false; } }
                    OP_NOT => { if !op_not(&mut stack) { return false; } }
                    OP_0NOTEQUAL => { if !op_0notequal(&mut stack) { return false; } }
                    OP_ADD => { if !op_add(&mut stack) { return false; } }
                    OP_SUB => { if !op_sub(&mut stack) { return false; } }
                    OP_MUL => { if !op_dup(&mut stack) { return false; } }
                    OP_BOOLAND => { if !op_booland(&mut stack) { return false; } }
                    OP_BOOLOR => { if !op_boolor(&mut stack) { return false; } }
                    OP_NUMEQUAL => { if !op_numequal(&mut stack) { return false; } }
                    OP_NUMEQUALVERIFY => { if !op_numequalverify(&mut stack) { return false; } }
                    OP_NUMNOTEQUAL => { if !op_numnotequal(&mut stack) { return false; } }
                    OP_LESSTHAN => { if !op_lessthan(&mut stack) { return false; } }
                    OP_GREATERTHAN => { if !op_greaterthan(&mut stack) { return false; } }
                    OP_LESSTHANOREQUAL => { if !op_lessthanorequal(&mut stack) { return false; } }
                    OP_GREATERTHANOREQUAL => { if !op_greaterthanorequal(&mut stack) { return false; } }
                    OP_MIN => { if !op_min(&mut stack) { return false; } }
                    OP_MAX => { if !op_max(&mut stack) { return false; } }
                    OP_WITHIN => { if !op_within(&mut stack) { return false; } }
                    OP_RIPEMD160 => { if !op_ripemd160(&mut stack) { return false; } }
                    OP_SHA1 => { if !op_sha1(&mut stack) { return false; } }
                    OP_SHA256 => { if !op_sha256(&mut stack) { return false; } }
                    OP_HASH160 => { if !op_hash160(&mut stack) { return false; } }
                    OP_HASH256 => { if !op_hash256(&mut stack) { return false; } }
                    OP_CODESEPARATOR => { if !op_codeseparator(&mut stack) { return false; } }
                    OP_CHECKSIG => { if !op_checksig(&mut stack, z) { return false; } }
                    OP_CHECKSIGVERIFY => { if !op_checksigverify(&mut stack, z) { return false; } }
                    OP_CHECKMULTISIG => { if !op_checkmultisig(&mut stack, z) { return false; } }
                    OP_CHECKMULTISIGVERIFY => { if !op_checkmultisigverify(&mut stack, z) { return false; } }

                    _ => {
                        panic!("UNKNOWN OP CODE {}", op_code);
                    }
                }
            } else {
                stack.push(cmd.clone());
                // p2sh form here. Previous row in p2sh is the push of the RedeemScript
                if cmds.len() == 3 && self.is_p2sh(&cmds) {
                    cmds.pop();
                    let h160 = cmds.pop();
                    cmds.pop();
                    if !op_hash160(&mut stack) {
                        return false
                    }
                    stack.push(h160.unwrap());
                    if !op_equal(&mut stack) {
                        return false
                    }
                    if !op_verify(&mut stack) {
                        println!("bad p2sh h160");
                        return false
                    }
                    let mut redeem_script: Vec<u8> = vec![];
                    redeem_script.extend(encode_varint(cmd.len() as u64).unwrap());
                    redeem_script.extend(cmd);
                    let mut cursor = Cursor::new(redeem_script);
                    cmds.extend(Script::parse(&mut cursor).unwrap().cmds);
                }
            }
        }
        if stack.len() == 0 {
            return false;
        }
        //if stack.pop().unwrap() == b"" {
        if stack.remove(0) == b"" {
            return false;
        }
        true
    }
    fn is_p2sh(&self, cmds: &Vec<Vec<u8>>) -> bool {
        cmds[0] == [0xa9] && cmds[1].len() == 20 && cmds[2] == [0x87]
    }
    pub fn p2pkh_script(h160: Vec<u8>) -> Self {
        let mut cmds: Vec<Vec<u8>> = Vec::new();
        cmds.push(vec![0x76]); // OP_DUP
        cmds.push(vec![0xa9]); // OP_HASH160
        cmds.push(h160);
        cmds.push(vec![0x88]); // OP_EQUALVERIFY
        cmds.push(vec![0xac]); // OP_CHECKSIG
        Script{cmds:cmds}
    }
}
impl Add for Script {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut combined = vec![];
        combined.extend(self.cmds.clone());
        combined.extend(other.cmds);
        Script::new(combined)
    }
}
impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op_code_names = op_code_names();
        let mut result = String::new();

        for cmd in &self.cmds {
            if cmd.len() == 1 {
                if !is_op(&cmd) {
                    panic!("OP not handled {:?}", cmd);
                }
                let op_code = cmd[0];
                result.push_str(&op_code_names[&op_code]);
            } else {
                result.push_str(
                    &cmd.iter()
                        .map(|byte| format!("{:02x}", byte))
                        .collect::<String>(),
                );
            }
            result.push(' ');
        }
        write!(f, "{}", result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use num::Num;
    #[test]
    fn test_parse() {

        let script_pubkey = hex::decode("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937").unwrap();
        let mut stream = Cursor::new(script_pubkey.clone());
        let script = Script::parse(stream.by_ref()).unwrap();
        println!("{}", script);
        let required = hex::decode("304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601").unwrap();
        assert_eq!(script.cmds[0], required);
        let required = hex::decode("035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937").unwrap();
        assert_eq!(script.cmds[1], required);

        // fake test OP_PUSHDATA2
        let script_pubkey = hex::decode("FD03014d0001aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let mut stream = Cursor::new(script_pubkey.clone());
        let script = Script::parse(stream.by_ref()).unwrap();
        println!("{}", script);
        let required = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        assert_eq!(script.cmds[0], required);

        // fake test OP_PUSHDATA1
        let script_pubkey = hex::decode("4F4c4caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        let mut stream = Cursor::new(script_pubkey.clone());
        let script = Script::parse(stream.by_ref()).unwrap();
        println!("{}", script);
        let required = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap();
        assert_eq!(script.cmds[0], required);
    }
    #[test]
    fn test_serialize() {

        let want = "6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937";
        let script_pubkey = hex::decode(want).unwrap();
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        println!("{}", script);
        assert_eq!(hex::encode(script.serialize()), want);

        let full = "fd03014d0001aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let script_pubkey = hex::decode(full).unwrap();
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        println!("{}", script);
        assert_eq!(hex::encode(script.serialize()), full);

        let full = "4e4c4caaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        let script_pubkey = hex::decode(full).unwrap();
        let mut script_pubkey = Cursor::new(script_pubkey);
        let script = Script::parse(&mut script_pubkey).unwrap();
        println!("{}", script);
        assert_eq!(hex::encode(script.serialize()), full);

    }
    #[test]
    fn test_is_op_1() {
        let mut cmds = vec![];
        cmds.push(76 as u8);
        assert!(is_op(&cmds) == true);
    }
    #[test]
    fn test_is_op_2() {
        let mut cmds = vec![];
        cmds.push(200 as u8);
        assert!(is_op(&cmds) == false);
    }
    #[test]
    fn test_asm() {
        // length 25, encode_varint -> 19
        let mut full_script = vec![];
        let hex = "76a914e94ba250bd0dcd459173f00d84433c1bb96747cd88ac";
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();
        full_script.extend(len);
        full_script.extend(script);
        let mut stream = Cursor::new(full_script);
        let script = Script::parse(&mut stream).unwrap();
        println!("{}", script);

    }
    #[test]
    fn test_asm_2() {
        // length 25, encode_varint -> 19
        let mut full_script = vec![];
        let hex = "00143f31a0455c60629a2add47c0be10f53fe873e848";
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();

        full_script.extend(len);
        full_script.extend(script);

        let mut stream = Cursor::new(full_script);
        let script = Script::parse(&mut stream).unwrap();
        println!("{}", script);

    }
    #[test]
    fn test_asm_3() {
        // length 25, encode_varint -> 19
        let mut full_script = vec![];
        // OP_DUP OP_DUP OP_MUL OP_ADD OP_6 OP_EQUAL
        let hex = "767695935687";
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();

        full_script.extend(len);
        full_script.extend(script);

        let mut stream = Cursor::new(full_script);
        let script = Script::parse(&mut stream).unwrap();
        println!("{}", script);

    }
    #[test]
    fn test_eval_1() {
        let mut full_script = vec![];
        let hex = "767695935687"; // OP_DUP OP_DUP OP_MUL OP_ADD OP_6 OP_EQUAL
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();
        full_script.extend(len);
        full_script.extend(script);
        let mut stream = Cursor::new(full_script);
        let script_pubkey = Script::parse(&mut stream).unwrap();

        let mut full_script = vec![];
        let hex = "52"; // --> 82, OP_2
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();
        full_script.extend(len);
        full_script.extend(script);
        let mut stream = Cursor::new(full_script);
        let script_sig = Script::parse(&mut stream).unwrap();

        let combined_script =  script_sig + script_pubkey;
        println!("COMBINED: {}", combined_script);
        let eval = combined_script.evaluate(&BigUint::from(0u32));
        println!("EVAL: {:?}", eval);
        assert_eq!(eval, true);
    }
    #[test]
    fn test_eval_2() {
        let mut full_script = vec![];
        let hex = "55935987"; // OP_5 OP_ADD OP_9 OP_EQUAL
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();
        full_script.extend(len);
        full_script.extend(script);
        let mut stream = Cursor::new(full_script);
        let script_pubkey = Script::parse(&mut stream).unwrap();

        let mut full_script = vec![];
        let hex = "54"; // OP_4
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();
        full_script.extend(len);
        full_script.extend(script);
        let mut stream = Cursor::new(full_script);
        let script_sig = Script::parse(&mut stream).unwrap();

        let combined_script =  script_sig + script_pubkey;
        println!("COMBINED: {}", combined_script);
        let eval = combined_script.evaluate(&BigUint::from(0u32));
        println!("EVAL: {:?}", eval);
        assert_eq!(eval, true);
    }
    #[test]
    fn test_p2pk() {
        let z = BigUint::from_str_radix("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d", 16).unwrap();
        let sec = "04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
        let sig = "3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601";

        let sec = hex::decode(sec).unwrap();
        let mut sec_v:Vec<Vec<u8>> = vec![];
        let check = hex::decode("ac").unwrap();
        sec_v.push(sec);
        sec_v.push(check);
        let script_pubkey = Script::new(sec_v);

        let sig = hex::decode(sig).unwrap();
        let mut sig_v:Vec<Vec<u8>> = vec![];
        sig_v.push(sig);
        let script_sig = Script::new(sig_v);

        let combined_script =  script_sig + script_pubkey;
        println!("COMBINED: {}", combined_script);
        let eval = combined_script.evaluate(&z);
        println!("EVAL: {:?}", eval);
        assert_eq!(eval, true);
    }
    #[test]
    fn test_asm_4() {
        //p2pk_script_pub_key = "76a91455ae51684c43435da751ac8d2173b2652eb6410588ac"
        //p2phk_script_sig = "483045022100c233c3a8a510e03ad18b0a24694ef00c78101bfd5ac075b8c1037952ce26e91e02205aa5f8f88f29bb4ad5808ebc12abfd26bd791256f367b04c6d955f01f28a7724012103f0609c81a45f8cab67fc2d050c21b1acd3d37c7acfd54041be6601ab4cef4f31"
        // length 25, encode_varint -> 19
        let mut full_script = vec![];
        // OP_DUP OP_HASH160 55ae51684c43435da751ac8d2173b2652eb64105 OP_EQUALVERIFY OP_CHECKSIG
        let hex = "76a91455ae51684c43435da751ac8d2173b2652eb6410588ac";
        let script = hex::decode(hex).unwrap();
        let len = encode_varint(script.len() as u64).unwrap();

        full_script.extend(len);
        full_script.extend(script);

        let mut stream = Cursor::new(full_script);
        let script = Script::parse(&mut stream).unwrap();
        println!("{}", script);

    }
}