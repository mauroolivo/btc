use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

pub fn hash160(bytes: &[u8]) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.update(Sha256::digest(bytes));
    let result = hasher.finalize();
    let mut hash: [u8; 20] = [0; 20];
    hash.copy_from_slice(&result);
    hash
}