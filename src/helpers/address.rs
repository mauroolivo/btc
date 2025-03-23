use crate::helpers::base58::base58_encode_checksum;

pub fn h160_to_p2pkh_address(h160: Vec<u8>, testnet: bool ) -> Vec<u8> {

    if h160.len() != 20 { panic!("h160 has no length 20"); }
    let mut result: Vec<u8> = vec![];
    if testnet {
        result.push(0x6f);
    } else {
        result.push(0x00);
    }
    result.extend_from_slice(&h160);
    base58_encode_checksum(result)
}
pub fn h160_to_p2sh_address(h160: Vec<u8>, testnet: bool ) -> Vec<u8> {

    if h160.len() != 20 { panic!("h160 has length 20"); }
    let mut result: Vec<u8> = vec![];
    if testnet {
        result.push(0xc4);
    } else {
        result.push(0x05);
    }
    result.extend_from_slice(&h160);
    base58_encode_checksum(result)
}
#[cfg(test)]
mod tests {
    use crate::helpers::address::{h160_to_p2pkh_address, h160_to_p2sh_address};

    #[test]
    fn test_p2pkh_address() {
        let h160 = hex::decode("74d691da1574e6b3c192ecfb52cc8984ee7b6c56").unwrap();
        let want = "1BenRpVUFK65JFWcQSuHnJKzc4M8ZP8Eqa".as_bytes().to_vec();
        assert_eq!(h160_to_p2pkh_address(h160, false), want);
        let h160 = hex::decode("74d691da1574e6b3c192ecfb52cc8984ee7b6c56").unwrap();
        let want = "mrAjisaT4LXL5MzE81sfcDYKU3wqWSvf9q".as_bytes().to_vec();
        assert_eq!(h160_to_p2pkh_address(h160, true), want);
    }
    #[test]
    fn test_p2sh_address() {
        let h160 = hex::decode("74d691da1574e6b3c192ecfb52cc8984ee7b6c56").unwrap();
        let want = "3CLoMMyuoDQTPRD3XYZtCvgvkadrAdvdXh".as_bytes().to_vec();
        assert_eq!(h160_to_p2sh_address(h160, false), want);
        let h160 = hex::decode("74d691da1574e6b3c192ecfb52cc8984ee7b6c56").unwrap();
        let want = "2N3u1R6uwQfuobCqbCgBkpsgBxvr1tZpe7B".as_bytes().to_vec();
        assert_eq!(h160_to_p2sh_address(h160, true), want);
    }
}