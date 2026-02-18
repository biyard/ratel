pub fn sha256_base64url(input: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(input.as_bytes());
    let digest = h.finalize();

    use base64::Engine as _;
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(digest)
}
