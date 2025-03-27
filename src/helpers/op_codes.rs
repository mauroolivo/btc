use std::collections::HashMap;
use num::BigUint;
use sha2::{Digest, Sha256};
use sha1::Sha1;
use ripemd::{Ripemd160};
use crate::point::Point;
// use crate::helpers::hex;
use crate::signature::Signature;

pub fn is_op(cmd: &Vec<u8>) -> bool {
    if cmd.len() == 1 {
        let list = op_code_names();
        if list.contains_key(&cmd[0]) {
            return true;
        }
    }
    false
}
pub fn op_code_names() -> HashMap<u8, &'static str> {
    let mut op_code_names = HashMap::new();
    op_code_names.insert(OP_0, "OP_0");
    op_code_names.insert(OP_PUSHDATA1, "OP_PUSHDATA1");
    op_code_names.insert(OP_PUSHDATA2, "OP_PUSHDATA2");
    op_code_names.insert(OP_PUSHDATA4, "OP_PUSHDATA4");
    op_code_names.insert(OP_1NEGATE, "OP_NEGATE");
    op_code_names.insert(OP_1, "OP_1");
    op_code_names.insert(OP_2, "OP_2");
    op_code_names.insert(OP_3, "OP_3");
    op_code_names.insert(OP_4, "OP_4");
    op_code_names.insert(OP_5, "OP_5");
    op_code_names.insert(OP_6, "OP_6");
    op_code_names.insert(OP_7, "OP_7");
    op_code_names.insert(OP_8, "OP_8");
    op_code_names.insert(OP_9, "OP_9");
    op_code_names.insert(OP_10, "OP_10");
    op_code_names.insert(OP_11, "OP_11");
    op_code_names.insert(OP_12, "OP_12");
    op_code_names.insert(OP_13, "OP_13");
    op_code_names.insert(OP_14, "OP_14");
    op_code_names.insert(OP_15, "OP_15");
    op_code_names.insert(OP_16, "OP_16");
    op_code_names.insert(OP_NOP, "OP_NOP");
    op_code_names.insert(OP_IF, "OP_IF");
    op_code_names.insert(OP_NOTIF, "OP_NOTIF");
    op_code_names.insert(OP_ELSE, "OP_ELSE");
    op_code_names.insert(OP_ENDIF, "OP_ENDIF");
    op_code_names.insert(OP_VERIFY, "OP_VERIFY");
    op_code_names.insert(OP_RETURN, "OP_RETURN");
    op_code_names.insert(OP_TOALTSTACK, "OP_TOALTSTACK");
    op_code_names.insert(OP_FROMALTSTACK, "OP_FROMALTSTACK");
    op_code_names.insert(OP_2DROP, "OP_2DROP");
    op_code_names.insert(OP_2DUP, "OP_2DUP");
    op_code_names.insert(OP_3DUP, "OP_3DUP");
    op_code_names.insert(OP_2OVER, "OP_2OVER");
    op_code_names.insert(OP_2ROT, "OP_2ROT");
    op_code_names.insert(OP_2SWAP, "OP_2SWAP");
    op_code_names.insert(OP_IFDUP, "OP_IFDUP");
    op_code_names.insert(OP_DEPTH, "OP_DEPTH");
    op_code_names.insert(OP_DROP, "OP_DROP");
    op_code_names.insert(OP_DUP, "OP_DUP");
    op_code_names.insert(OP_NIP, "OP_NIP");
    op_code_names.insert(OP_OVER, "OP_OVER");
    op_code_names.insert(OP_PICK, "OP_PICK");
    op_code_names.insert(OP_ROLL, "OP_ROLL");
    op_code_names.insert(OP_ROT, "OP_ROT");
    op_code_names.insert(OP_SWAP, "OP_SWAP");
    op_code_names.insert(OP_TUCK, "OP_TUCK");
    op_code_names.insert(OP_SIZE, "OP_SIZE");
    op_code_names.insert(OP_EQUAL, "OP_EQUAL");
    op_code_names.insert(OP_EQUALVERIFY, "OP_EQUALVERIFY");
    op_code_names.insert(OP_1ADD, "OP_1ADD");
    op_code_names.insert(OP_1SUB, "OP_1SUB");
    op_code_names.insert(OP_NEGATE, "OP_NEGATE");
    op_code_names.insert(OP_ABS, "OP_ABS");
    op_code_names.insert(OP_NOT, "OP_NOT");
    op_code_names.insert(OP_0NOTEQUAL, "OP_0NOTEQUAL");
    op_code_names.insert(OP_ADD, "OP_ADD");
    op_code_names.insert(OP_SUB, "OP_SUB");
    op_code_names.insert(OP_MUL, "OP_MUL");
    op_code_names.insert(OP_BOOLAND, "OP_BOOLAND");
    op_code_names.insert(OP_BOOLOR, "OP_BOOLOR");
    op_code_names.insert(OP_NUMEQUAL, "OP_NUMEQUAL");
    op_code_names.insert(OP_NUMEQUALVERIFY, "OP_NUMEQUALVERIFY");
    op_code_names.insert(OP_NUMNOTEQUAL, "OP_NUMNOTEQUAL");
    op_code_names.insert(OP_LESSTHAN, "OP_LESSTHAN");
    op_code_names.insert(OP_GREATERTHAN, "OP_GREATERTHAN");
    op_code_names.insert(OP_GREATERTHANOREQUAL, "OP_GREATERTHANOREQUAL");
    op_code_names.insert(OP_MIN, "OP_MIN");
    op_code_names.insert(OP_MAX, "OP_MAX");
    op_code_names.insert(OP_WITHIN, "OP_WITHIN");
    op_code_names.insert(OP_RIPEMD160, "OP_RIPEMD160");
    op_code_names.insert(OP_SHA1, "OP_SHA1");
    op_code_names.insert(OP_SHA256, "OP_SHA256");
    op_code_names.insert(OP_HASH160, "OP_HASH160");
    op_code_names.insert(OP_HASH256, "OP_HASH256");
    op_code_names.insert(OP_CODESEPARATOR, "OP_CODESEPARATOR");
    op_code_names.insert(OP_CHECKSIG, "OP_CHECKSIG");
    op_code_names.insert(OP_CHECKSIGVERIFY, "OP_CHECKSIGVERIFY");
    op_code_names.insert(OP_CHECKMULTISIG, "OP_CHECKMULTISIG");

    op_code_names
}
pub const OP_0: u8 = 0;
pub const OP_PUSHDATA1: u8 = 76;
pub const OP_PUSHDATA2: u8 = 77;
pub const OP_PUSHDATA4: u8 = 78;
pub const OP_1NEGATE: u8 = 79;
pub const OP_1: u8 = 81;
pub const OP_2: u8 = 82;
pub const OP_3: u8 = 83;
pub const OP_4: u8 = 84;
pub const OP_5: u8 = 85;
pub const OP_6: u8 = 86;
pub const OP_7: u8 = 87;
pub const OP_8: u8 = 88;
pub const OP_9: u8 = 89;
pub const OP_10: u8 = 90;
pub const OP_11: u8 = 91;
pub const OP_12: u8 = 92;
pub const OP_13: u8 = 93;
pub const OP_14: u8 = 94;
pub const OP_15: u8 = 95;
pub const OP_16: u8 = 96;
pub const OP_NOP: u8 = 97;
pub const OP_IF: u8 = 99;
pub const OP_NOTIF: u8 = 100;
pub const OP_ELSE: u8 = 103;
pub const OP_ENDIF: u8 = 104;
pub const OP_VERIFY: u8 = 105;
pub const OP_RETURN: u8 = 106;
pub const OP_TOALTSTACK: u8 = 107;
pub const OP_FROMALTSTACK: u8 = 108;
pub const OP_2DROP: u8 = 109;
pub const OP_2DUP: u8 = 110;
pub const OP_3DUP: u8 = 111;
pub const OP_2OVER: u8 = 112;
pub const OP_2ROT: u8 = 113;
pub const OP_2SWAP: u8 = 114;
pub const OP_IFDUP: u8 = 115;
pub const OP_DEPTH: u8 = 116;
pub const OP_DROP: u8 = 117;
pub const OP_DUP: u8 = 118;
pub const OP_NIP: u8 = 119;
pub const OP_OVER: u8 = 120;
pub const OP_PICK: u8 = 121;
pub const OP_ROLL: u8 = 122;
pub const OP_ROT: u8 = 123;
pub const OP_SWAP: u8 = 124;
pub const OP_TUCK: u8 = 125;
pub const OP_SIZE: u8 = 130;
pub const OP_EQUAL: u8 = 135;
pub const OP_EQUALVERIFY: u8 = 136;
pub const OP_1ADD: u8 = 139;
pub const OP_1SUB: u8 = 140;
pub const OP_NEGATE: u8 = 143;
pub const OP_ABS: u8 = 144;
pub const OP_NOT: u8 = 145;
pub const OP_0NOTEQUAL: u8 = 146;
pub const OP_ADD: u8 = 147;
pub const OP_SUB: u8 = 148;
pub const OP_MUL: u8 = 149;
pub const OP_BOOLAND: u8 = 154;
pub const OP_BOOLOR: u8 = 155;
pub const OP_NUMEQUAL: u8 = 156;
pub const OP_NUMEQUALVERIFY: u8 = 157;
pub const OP_NUMNOTEQUAL: u8 = 158;
pub const OP_LESSTHAN: u8 = 159;
pub const OP_GREATERTHAN: u8 = 160;
pub const OP_LESSTHANOREQUAL: u8 = 161;
pub const OP_GREATERTHANOREQUAL: u8 = 162;
pub const OP_MIN: u8 = 163;
pub const OP_MAX: u8 = 164;
pub const OP_WITHIN: u8 = 165;
pub const OP_RIPEMD160: u8 = 166;
pub const OP_SHA1: u8 = 167;
pub const OP_SHA256: u8 = 168;
pub const OP_HASH160: u8 = 169;
pub const OP_HASH256: u8 = 170;
pub const OP_CODESEPARATOR: u8 = 171;
pub const OP_CHECKSIG: u8 = 172;
pub const OP_CHECKSIGVERIFY: u8 = 173;
pub const OP_CHECKMULTISIG: u8 = 174;
pub const OP_CHECKMULTISIGVERIFY: u8 = 175;
pub const OP_NOOP1: u8 = 176;
pub const OP_CHECKLOCKTIMEVERIFY: u8 = 177;
pub const OP_CHECKSEQUENCEVERIFY: u8 = 178;

pub fn encode_num(num: i32) -> Vec<u8> {
    if num == 0 {
        return vec![];
    }
    let abs_num = num.abs();
    let negative = num < 0;
    let mut result = Vec::new();
    let mut remaining = abs_num;
    while remaining > 0 {
        result.push((remaining & 0xff) as u8);
        remaining >>= 8;
    }
    if result.last().unwrap() & 0x80 != 0 {
        if negative {
            result.push(0x80);
        } else {
            result.push(0);
        }
    } else if negative {
        let last = result.last_mut().unwrap();
        *last |= 0x80;
    }
    result
}

pub fn decode_num(element: &[u8]) -> i32 {
    if element.is_empty() {
        return 0;
    }
    let big_endian = element.iter().rev().cloned().collect::<Vec<_>>();
    let negative = big_endian[0] & 0x80 != 0;
    let mut result = if negative {
        (big_endian[0] & 0x7f) as i32
    } else {
        big_endian[0] as i32
    };
    for &c in &big_endian[1..] {
        result <<= 8;
        result += c as i32;
    }
    if negative {
        -result
    } else {
        result
    }
}

pub fn op_0(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(0));
    true
}
pub fn op_1negate(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(-1));
    true
}
pub fn op_1(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(1));
    true
}
pub fn op_2(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(2));
    true
}
pub fn op_3(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(3));
    true
}
pub fn op_4(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(4));
    true
}
pub fn op_5(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(5));
    true
}
pub fn op_6(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(6));
    true
}
pub fn op_7(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(7));
    true
}
pub fn op_8(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(8));
    true
}
pub fn op_9(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(9));
    true
}
pub fn op_10(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(10));
    true
}
pub fn op_11(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(11));
    true
}
pub fn op_12(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(12));
    true
}
pub fn op_13(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(13));
    true
}
pub fn op_14(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(14));
    true
}
pub fn op_15(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(15));
    true
}
pub fn op_16(stack: &mut Vec<Vec<u8>>) -> bool {
    stack.push(encode_num(16));
    true
}
pub fn op_nop(_stack: &mut Vec<Vec<u8>>) -> bool {
    true
}
pub fn op_if(stack: &mut Vec<Vec<u8>>, items: &mut Vec<u8>) -> bool {
    if stack.len() < 1 {
        return false;
    }

    let mut true_items = Vec::new();
    let mut false_items = Vec::new();
    let mut current_array = &mut true_items;
    let mut found = false;
    let mut num_endifs_needed = 1;

    while !items.is_empty() {
        let item = items.remove(0);
        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            }
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_items;
            }
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);
                }
            }
            _ => {
                current_array.push(item);
            }
        }
    }

    if !found {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&element) == 0 {
        items.splice(0..0, false_items.into_iter());
    } else {
        items.splice(0..0, true_items.into_iter());
    }

    true
}
pub fn op_notif(stack: &mut Vec<Vec<u8>>, items: &mut Vec<u8>) -> bool {
    if stack.len() < 1 {
        return false;
    }

    let mut true_items = Vec::new();
    let mut false_items = Vec::new();
    let mut current_array = &mut true_items;
    let mut found = false;
    let mut num_endifs_needed = 1;

    while !items.is_empty() {
        let item = items.remove(0);
        match item {
            99 | 100 => {
                num_endifs_needed += 1;
                current_array.push(item);
            }
            103 if num_endifs_needed == 1 => {
                current_array = &mut false_items;
            }
            104 => {
                if num_endifs_needed == 1 {
                    found = true;
                    break;
                } else {
                    num_endifs_needed -= 1;
                    current_array.push(item);
                }
            }
            _ => {
                current_array.push(item);
            }
        }
    }

    if !found {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&element) == 0 {
        items.splice(0..0, true_items.into_iter());
    } else {
        items.splice(0..0, false_items.into_iter());
    }

    true
}
pub fn op_else(stack: &mut Vec<Vec<u8>>, items: &mut Vec<u8>) -> bool {
    println!("{}", stack.len());
    println!("{:?}", items);
    panic!("op_else not implemented")
}
pub fn op_endif(stack: &mut Vec<Vec<u8>>, items: &mut Vec<u8>) -> bool {
    println!("{}", stack.len());
    println!("{:?}", items);
    panic!("op_endif not implemented")
}
pub fn op_verify(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }

    let element = stack.pop().unwrap();
    if decode_num(&element) == 0 {
        return false;
    }

    true
}
pub fn op_return(_stack: &mut Vec<Vec<u8>>) -> bool {
    return false;
}
pub fn op_toaltstack(stack: &mut Vec<Vec<u8>>, altstack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    altstack.push(item);
    true
}
pub fn op_fromaltstack(stack: &mut Vec<Vec<u8>>, altstack: &mut Vec<Vec<u8>>) -> bool {
    if altstack.is_empty() {
        return false;
    }
    let item = altstack.pop().unwrap();
    stack.push(item);
    true
}
pub fn op_2drop(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    stack.pop();
    stack.pop();
    true
}
pub fn op_2dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack[stack.len() - 2].clone();
    let item2 = stack[stack.len() - 1].clone();
    stack.push(item1);
    stack.push(item2);
    true
}
pub fn op_3dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 3 {
        return false;
    }
    let item1 = stack[stack.len() - 3].clone();
    let item2 = stack[stack.len() - 2].clone();
    let item3 = stack[stack.len() - 1].clone();
    stack.push(item1);
    stack.push(item2);
    stack.push(item3);
    true
}
pub fn op_2over(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 4 {
        return false;
    }
    let item1 = stack[stack.len() - 4].clone();
    let item2 = stack[stack.len() - 3].clone();
    stack.push(item1);
    stack.push(item2);
    true
}
pub fn op_2rot(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 6 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    let item4 = stack.pop().unwrap();
    let item5 = stack.pop().unwrap();
    let item6 = stack.pop().unwrap();
    stack.push(item3);
    stack.push(item4);
    stack.push(item1);
    stack.push(item2);
    stack.push(item5);
    stack.push(item6);
    true
}
pub fn op_2swap(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 4 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    let item4 = stack.pop().unwrap();
    stack.push(item3);
    stack.push(item4);
    stack.push(item1);
    stack.push(item2);
    true
}
pub fn op_ifdup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let item = stack[stack.len() - 1].clone();
    if decode_num(&item) != 0 {
        stack.push(item);
    }
    true
}
pub fn op_depth(stack: &mut Vec<Vec<u8>>) -> bool {
    let depth = stack.len() as i32;
    stack.push(encode_num(depth));
    true
}
pub fn op_drop(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    stack.pop();
    true
}
pub fn op_dup(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        println!("op_dup called with empty stack");
        return false;
    }
    let item = stack[stack.len() - 1].clone();
    stack.push(item);
    true
}
pub fn op_nip(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    stack.remove(stack.len() - 2);
    true
}
pub fn op_over(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item = stack[stack.len() - 2].clone();
    stack.push(item);
    true
}
pub fn op_pick(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let item = stack.pop().unwrap();
    let n = decode_num(&item) as usize;
    if stack.len() < n {
        return false;
    }
    let item = stack[stack.len() - n].clone();
    stack.push(item);
    true
}
pub fn op_roll(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 1 {
        return false;
    }
    let item = stack.pop().unwrap();
    let n = decode_num(&item) as usize;
    if stack.len() < n {
        return false;
    }
    let item = stack.remove(stack.len() - n);
    stack.push(item);
    true
}
pub fn op_rot(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 3 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    stack.push(item2);
    stack.push(item1);
    stack.push(item3);
    true
}
pub fn op_swap(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    stack.push(item1);
    stack.push(item2);
    true
}
pub fn op_tuck(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    stack.push(item1.clone());
    stack.push(item2);
    stack.push(item1);
    true
}
pub fn op_size(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let size = item.len() as i32;
    stack.push(encode_num(size));
    true
}
pub fn op_equal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let result = if item1 == item2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_equalverify(stack: &mut Vec<Vec<u8>>) -> bool {
    if !op_equal(stack) {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    if decode_num(&item) == 0 {
        return false;
    }
    true
}
pub fn op_1add(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(num + 1));
    true
}
pub fn op_1sub(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(num - 1));
    true
}
pub fn op_negate(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(-num));
    true
}
pub fn op_abs(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    stack.push(encode_num(num.abs()));
    true
}
pub fn op_not(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    let result = if num == 0 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_0notequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let num = decode_num(&item);
    let result = if num == 0 { 0 } else { 1 };
    stack.push(encode_num(result));
    true
}
pub fn op_add(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    stack.push(encode_num(num1 + num2));
    true
}
pub fn op_sub(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    stack.push(encode_num(num1 - num2));
    true
}
pub fn op_mul(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = decode_num(stack.pop().unwrap().as_slice());
    let item2 = decode_num(stack.pop().unwrap().as_slice());
    stack.push(encode_num(item1 * item2));
    true
}
pub fn op_booland(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 != 0 && num2 != 0 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_boolor(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 != 0 || num2 != 0 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_numequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 == num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_numequalverify(stack: &mut Vec<Vec<u8>>) -> bool {
    if !op_numequal(stack) {
        return false;
    }
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    if decode_num(&item) == 0 {
        return false;
    }
    true
}
pub fn op_numnotequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 != num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_lessthan(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 < num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_greaterthan(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 > num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_lessthanorequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 <= num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_greaterthanorequal(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 >= num2 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_min(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 < num2 { num1 } else { num2 };
    stack.push(encode_num(result));
    true
}
pub fn op_max(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 2 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let result = if num1 > num2 { num1 } else { num2 };
    stack.push(encode_num(result));
    true
}
pub fn op_within(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.len() < 3 {
        return false;
    }
    let item1 = stack.pop().unwrap();
    let item2 = stack.pop().unwrap();
    let item3 = stack.pop().unwrap();
    let num1 = decode_num(&item1);
    let num2 = decode_num(&item2);
    let num3 = decode_num(&item3);
    let result = if num2 <= num1 && num1 < num3 { 1 } else { 0 };
    stack.push(encode_num(result));
    true
}
pub fn op_ripemd160(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let item = stack.pop().unwrap();
    let hash = Ripemd160::digest(&item);
    stack.push(hash.to_vec());
    true
}
pub fn op_sha1(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha1::new();
    hasher.update(&element);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}
pub fn op_sha256(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&element);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}
pub fn op_hash160(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&element);
    let result = hasher.finalize();
    let mut hasher = Ripemd160::new();
    hasher.update(&result);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}
pub fn op_hash256(stack: &mut Vec<Vec<u8>>) -> bool {
    if stack.is_empty() {
        return false;
    }
    let element = stack.pop().unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&element);
    let result = hasher.finalize();
    let mut hasher = Sha256::new();
    hasher.update(&result);
    let result = hasher.finalize();
    stack.push(result.to_vec());
    true
}
pub fn op_codeseparator(stack: &mut Vec<Vec<u8>>) -> bool {
    println!("{}", stack.len());
    panic!("op_codeseparator not implemented")
}
pub fn op_checksig(stack: &mut Vec<Vec<u8>>, z: &BigUint) -> bool {
    if stack.len() < 2 {
        return false;
    }
    // the top element of the stack is the SEC pubkey
    // the top element is the last added
    // sec is the last added, pop takes the last!
    let sec = stack.pop().unwrap();
    // the next element of the stack is the DER signature
    let mut der = stack.pop().unwrap();

    // take off the last byte of the signature as that's the hash_type
    der.pop();

    let point1 = Point::parse(&sec);
    let point2 = Signature::parse(&der);

    if point1.verify(z, &point2.unwrap()) == true {
        stack.push(encode_num(1));
    } else {
        stack.push(encode_num(0));
    }
    true
}
pub fn op_checksigverify(stack: &mut Vec<Vec<u8>>, z: &BigUint) -> bool {
    op_checksig(stack, z) && op_verify(stack)
}
pub fn op_checkmultisig(stack: &mut Vec<Vec<u8>>, z: &BigUint) -> bool {
    if stack.len() < 1 {
        return false;
    }
    // m of n : m signatures oof n pub keys
    let n = decode_num(&stack.pop().unwrap()) as usize;
    if stack.len() < (n + 1) {
        return false;
    }
    let mut sec_pubkeys: Vec<Vec<u8>> = vec![];
    for _ in 0..n {
        sec_pubkeys.push(stack.pop().unwrap())
    }

    let m = decode_num(&stack.pop().unwrap()) as usize;
    if stack.len() < (m + 1) {
        return false;
    }
    let mut der_signatures: Vec<Vec<u8>> = vec![];
    for _ in 0..m {
        let mut der_signature = stack.pop().unwrap();
        der_signature.pop(); // der signature is assumed to be signed with SIGHASH_ALL
        der_signatures.push(der_signature);
    }

    // OP_CHECKMULTISIG bug
    stack.pop();

    let mut points: Vec<Point> = vec![];
    let mut sigs: Vec<Signature> = vec![];

    for sec_pubkey in sec_pubkeys {
        points.push(Point::parse(sec_pubkey.as_slice()));
    }
    for der in der_signatures {
        sigs.push(Signature::parse(&der).unwrap());
    }

    for sig in sigs {
        // if we have no more points, signatures are no good
        if points.len() == 0 {
            return false;
        }
        while points.len() > 0 {
            let point = points.remove(0);
            if point.verify(z, &sig) {
                break
            } else { println!("not verfied"); }
        }
    }
    stack.push(encode_num(1));
    true
}
pub fn op_checkmultisigverify(stack: &mut Vec<Vec<u8>>, z: &BigUint) -> bool {
    op_checkmultisig(stack, z) && op_verify(stack)
}
pub fn op_noop1(_stack: &mut Vec<Vec<u8>>) -> bool {
    panic!("op_noop1 not implemented")
}
pub fn op_checklocktimeverify(stack: &mut Vec<Vec<u8>>, locktime: u32, sequence: u32) -> bool {
    if sequence == 0xffffffff {
        return false;
    }
    if stack.len() < 1 {
        return false;
    }
    let element = decode_num(stack.last().unwrap());
    if element < 0 {
        return false;
    }
    if element < 500_000_000 && locktime > 500_000_000 {
        return false;
    }
    if locktime < element as u32 {
        return false;
    }
    true
}
pub fn op_checksequenceverify(stack: &mut Vec<Vec<u8>>, version: u32, sequence: u32) -> bool {
    if sequence & (1 << 31) == (1 << 31) {
        return false;
    }
    if stack.len() < 1 {
        return false;
    }
    let element = decode_num(stack.last().unwrap());
    if element < 0 {
        return false;
    }
    if element as u32 & (1 << 31) == (1 << 31) {
        if version < 2 {
            return false;
        } else if sequence & (1 << 31) == (1 << 31) {
            return false;
        } else if element as u32 & (1 << 22) != sequence & (1 << 22) {
            return false;
        } else if element as u32 & 0xffff > sequence & 0xffff {
            return false;
        }
    }
    true
}
#[cfg(test)]
mod tests {
    use super::*;
    use num::Num;
    #[test]
    fn test_op_hash160() {
        let mut stack: Vec<Vec<u8>> = vec![b"hello world".to_vec()];
        assert_eq!(op_hash160(&mut stack), true);
        assert_eq!(hex::encode(stack[0].clone()), "d7d5ee7824ff93f94c3055af9382c86c68b5ca92");
    }
    #[test]
    fn test_op_checksig() {

        let z = BigUint::from_str_radix("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d", 16).unwrap();
        let sec = hex::decode("04887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34").unwrap();
        let sig = hex::decode("3045022000eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c022100c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab601").unwrap();
        let mut stack: Vec<Vec<u8>> = vec![sig, sec];
        assert_eq!(op_checksig(&mut stack, &z), true);
        assert_eq!(decode_num(stack[0].as_slice()), 1);
    }
    #[test]
    fn test_op_checkmultisig() {
        let z = BigUint::from_str_radix("e71bfa115715d6fd33796948126f40a8cdd39f187e4afb03896795189fe1423c", 16).unwrap();
        let sig1 = hex::decode("3045022100dc92655fe37036f47756db8102e0d7d5e28b3beb83a8fef4f5dc0559bddfb94e02205a36d4e4e6c7fcd16658c50783e00c341609977aed3ad00937bf4ee942a8993701").unwrap();
        let sig2 = hex::decode("3045022100da6bee3c93766232079a01639d07fa869598749729ae323eab8eef53577d611b02207bef15429dcadce2121ea07f233115c6f09034c0be68db99980b9a6c5e75402201").unwrap();
        let sec1 = hex::decode("022626e955ea6ea6d98850c994f9107b036b1334f18ca8830bfff1295d21cfdb70").unwrap();
        let sec2 = hex::decode("03b287eaf122eea69030a0e9feed096bed8045c8b98bec453e1ffac7fbdbd4bb71").unwrap();
        let mut stack: Vec<Vec<u8>> = vec![];
        stack.push(b"".to_vec());
        stack.push(sig1);
        stack.push(sig2);
        stack.push(b"\x02".to_vec());
        stack.push(sec1);
        stack.push(sec2);
        stack.push(b"\x02".to_vec());
        assert_eq!(op_checkmultisig(&mut stack, &z), true);
        assert_eq!(decode_num(stack[0].as_slice()), 1);
    }
}