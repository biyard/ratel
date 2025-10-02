use sha3::{Digest, Sha3_256};

pub fn hash_password(password: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}
