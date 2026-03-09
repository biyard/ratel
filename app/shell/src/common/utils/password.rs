use sha2::Sha256;
use sha3::{Digest, Sha3_256};

pub fn hash_password(password: &str) -> String {
    // In Frontend(React), they use ethers.js SHA256 function.
    // So we use sha3_256 here.
    let mut sha256 = Sha256::new();
    sha256.update(password.as_bytes()); // UTF-8
    let digest = sha256.finalize();
    let hash = format!("0x{}", hex::encode(digest));

    let mut hasher = Sha3_256::new();
    hasher.update(hash);
    let result = hasher.finalize();
    hex::encode(result)
}
