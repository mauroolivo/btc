use sha2::{Digest, Sha256};
pub fn hash256(bytes: &[u8]) -> [u8; 32] {

    let hash = Sha256::digest(Sha256::digest(&bytes));
    let mut result: [u8; 32] = [0; 32];
    result.copy_from_slice(&hash);
    result
}