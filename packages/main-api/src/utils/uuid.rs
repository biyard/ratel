pub fn sorted_uuid() -> String {
    use uuid::Uuid;
    let uuid = Uuid::new_v4();
    let ts = chrono::Utc::now().timestamp_millis() as u64;

    let mut bytes = [0u8; 16];
    bytes[..8].copy_from_slice(&ts.to_be_bytes());
    bytes[8..].copy_from_slice(uuid.as_bytes());

    Uuid::from_bytes(bytes).to_string()
}
