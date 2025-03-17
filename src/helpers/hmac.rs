use sha2::Sha256;
use hmac::{Hmac, KeyInit, Mac};
use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use base64::prelude::BASE64_STANDARD;

pub fn hmac_signature(key: &str, msg: &str) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(key.as_bytes()).unwrap();
    mac.update(&msg.as_bytes());
    let code_bytes = mac.finalize().into_bytes();

    BASE64_STANDARD.encode(&code_bytes)
}
#[cfg(test)]
mod tests {
    use crate::helpers::hmac::hmac_signature;

    #[test]
    fn test_hmac_signature() {
        let key = "secret";
        let msg = "message";

        let signature = hmac_signature(key, msg);

        let expected = "i19IcCmVwVmMVz2x4hhmqbgl1KeU0WnXBgoDYFeWNgs=";

        println!("signature: {}", signature);
        assert_eq!(signature, expected);
    }
}