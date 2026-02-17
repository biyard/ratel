use common::models::*;
use common::*;

pub fn new_user(
    display_name: String,
    email: String,
    profile_url: String,
    term_agreed: bool,
    informed_agreed: bool,
    user_type: UserType,
    username: String,
    password: Option<String>,
) -> User {
    let uid = uuid::Uuid::new_v4().to_string();
    let pk = Partition::User(uid);
    let sk = EntityType::User;

    let now = chrono::Utc::now().timestamp_millis();

    User {
        pk,
        sk,
        created_at: now,
        updated_at: now,
        display_name,
        email,
        profile_url,
        term_agreed,
        informed_agreed,
        user_type,
        username,
        password,
        ..Default::default()
    }
}

pub fn new_phone_user(phone: String) -> User {
    let uid = uuid::Uuid::new_v4().to_string();
    let pk = Partition::User(uid);
    let sk = EntityType::User;

    let now = chrono::Utc::now().timestamp_millis();
    let display_name = format!("user{}", now);

    User {
        pk,
        sk,
        created_at: now,
        updated_at: now,
        display_name: display_name.clone(),
        email: phone,
        profile_url: String::new(),
        term_agreed: true,
        informed_agreed: false,
        user_type: UserType::Individual,
        username: display_name,
        password: None,
        ..Default::default()
    }
}
