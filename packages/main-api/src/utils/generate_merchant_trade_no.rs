use rand::{Rng, distr::Alphanumeric};

fn sanitize_alnum(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect()
}

pub fn gen_merchant_trade_no(plan_code: &str) -> String {
    let base = sanitize_alnum(plan_code);
    let rand_tag: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let mut mt = format!("{}{}", base, rand_tag);
    if mt.len() > 32 {
        mt.truncate(32);
    }
    if mt.is_empty() {
        mt = rand_tag;
    }
    mt
}
