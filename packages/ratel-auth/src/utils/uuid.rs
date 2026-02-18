// Migrated from packages/main-api/src/utils/uuid.rs
pub fn sorted_uuid() -> String {
    use uuid::Uuid;
    let uid = Uuid::now_v7();

    uid.to_string()
}
