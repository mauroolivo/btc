use bech32::{hrp, segwit, Hrp, Bech32m};
use num::BigUint;

pub enum SegwitVersion {
    version_0,
    version_1,
}
pub fn bech32_segwit_encode(hash: Vec<u8>, segwit_version: SegwitVersion) -> String {
    let version = match segwit_version {
        SegwitVersion::version_0 => segwit::VERSION_0,
        SegwitVersion::version_1 => segwit::VERSION_1,
    };
    segwit::encode(hrp::BC, version, &hash).expect("valid witness version and program")
}
#[cfg(test)]
mod tests {
    const DATA: [u8; 20] = [0xab; 20]; // Arbitrary data to be encoded.
    const STRING: &str = "abc14w46h2at4w46h2at4w46h2at4w46h2at958ngu";
    const TAP_ADDR: &str = "bc1p4w46h2at4w46h2at4w46h2at4w46h2at5kreae";

    use super::*;
    #[test]
    fn debug_bech32() {
        // p2wpkh
        let data = vec![96, 214, 225, 172, 224, 2, 253, 142, 175, 50, 21, 11, 245, 124, 133, 212, 76, 22, 225, 106];
        let address = bech32_segwit_encode(data, SegwitVersion::version_0);
        assert_eq!(address, "bc1qvrtwrt8qqt7catejz59l2ly963xpdct2j370g3".to_string());

        // p2tr
        let data2 = vec![113, 5, 116, 56, 145, 120, 21, 46, 95, 188, 82, 85, 138, 130, 24, 236, 26, 223, 230, 132, 219, 139, 191, 67, 0, 165, 177, 165, 44, 246, 32, 172];
        let address2 = bech32_segwit_encode(data2, SegwitVersion::version_1);
        assert_eq!(address2, "bc1pwyzhgwy30q2juhau2f2c4qscasddle5ymw9m7scq5kc62t8kyzkqyz059k".to_string());

        // p2sh
        let data3 = vec![101, 249, 26, 83, 203, 113, 32, 5, 125, 179, 211, 120, 189, 15, 125, 148, 65, 103, 212, 58, 125, 203, 255, 21, 214, 175, 196, 130, 63, 29, 62, 211];
        let address3 = bech32_segwit_encode(data3, SegwitVersion::version_0);
        assert_eq!(address3, "bc1qvhu3557twysq2ldn6dut6rmaj3qk04p60h9l79wk4lzgy0ca8mfsnffz65".to_string());

    }
    #[test]
    fn encode_bech32() {
        // Encode arbitrary data using "abc" as the human-readable part and append a bech32m checksum.
        let hrp = Hrp::parse("abc").expect("valid hrp");
        let string = bech32::encode::<Bech32m>(hrp, &DATA).expect("failed to encode string");
        assert_eq!(string, STRING);

        // Encode arbitrary data as a Bitcoin taproot address.
        let taproot_address = segwit::encode(hrp::BC, segwit::VERSION_1, &DATA).expect("valid witness version and program");
        assert_eq!(taproot_address, TAP_ADDR);

        // No-alloc: Encode without allocating (ignoring that String::new() allocates :).
        let mut buf = String::new();
        bech32::encode_to_fmt::<Bech32m, String>(&mut buf, hrp, &DATA).expect("failed to encode to buffer");
        assert_eq!(buf, STRING);
    }
    #[test]
    fn decode_bech32() {
        use bech32::primitives::decode::{CheckedHrpstring, SegwitHrpstring};
        use bech32::{hrp, segwit, Hrp, Bech32m};

        const DATA: [u8; 20] = [0xab; 20]; // Arbitrary data to be encoded.
        const STRING: &str = "abc14w46h2at4w46h2at4w46h2at4w46h2at958ngu";
        const TAP_ADDR: &str = "bc1p4w46h2at4w46h2at4w46h2at4w46h2at5kreae";

        // Decode a bech32 encoded string that includes a bech32/bech32m checksum.
        //
        // The input address MUST include a valid bech32 or bech32m checksum, for individual specific
        // checksum algorithms see [`decode_bech32`], [`decode_bech32m`], [`decode_no_checksum`] or use
        // the [`primitives::decode::CheckedHrpstring`] type directly.
        let (hrp, data) = bech32::decode(&STRING).expect("failed to decode");
        assert_eq!(hrp, Hrp::parse("abc").unwrap());
        assert_eq!(data, DATA);

        // Decode a Bitcoin taproot address.
        let (_hrp, _version, program) = segwit::decode(&TAP_ADDR).expect("valid address");
        assert_eq!(program, DATA);

        // No-alloc: Decode a bech32m checksummed address without allocating.
        let p = CheckedHrpstring::new::<Bech32m>(&STRING).expect("failed to parse string");
        assert_eq!(hrp, p.hrp());
        assert!(p.byte_iter().eq(DATA.iter().map(|&b| b))); // We yield bytes not references.

        // No-alloc: Decode a taproot address without allocating.
        let taproot = SegwitHrpstring::new(&TAP_ADDR).expect("valid address");
        // Do something with the encoded data.
        let _ = taproot.byte_iter();
    }
}