use uuid::Uuid;

pub fn generate_referral_code() -> String {
    let uuid = Uuid::new_v4();
    encode_base32(&uuid.as_bytes()[..5])
}

fn encode_base32(bytes: &[u8]) -> String {
    const ALPHABET: &[u8] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

    let mut result = String::new();
    let mut buffer = 0u64;
    let mut bits_in_buffer = 0;

    for &byte in bytes {
        buffer = (buffer << 8) | (byte as u64);
        bits_in_buffer += 8;

        while bits_in_buffer >= 5 {
            let index = (buffer >> (bits_in_buffer - 5)) & 0x1F;
            result.push(ALPHABET[index as usize] as char);
            bits_in_buffer -= 5;
        }
    }

    if bits_in_buffer > 0 {
        let index = (buffer << (5 - bits_in_buffer)) & 0x1F;
        result.push(ALPHABET[index as usize] as char);
    }

    result
}
