use rand::Rng;

pub fn generate_random_code() -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let code: String = (0..6)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    code
}

pub fn generate_random_numeric_code() -> String {
    let charset = b"0123456789";
    let mut rng = rand::rng();
    let code: String = (0..6)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    code
}
