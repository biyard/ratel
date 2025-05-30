#[derive(Clone)]
pub struct User {
    pub name: String,
    pub position: String,
    pub is_following: bool,
}

impl User {
    pub fn new(name: &str, position: &str) -> Self {
        Self {
            name: name.to_string(),
            position: position.to_string(),
            is_following: false,
        }
    }
}