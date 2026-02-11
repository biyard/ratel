#[derive(Debug)]
pub struct Config {
    pub env: &'static str,
    pub private_bucket_name: &'static str,
    pub dynamo_endpoint: Option<&'static str>,
}

impl Default for Config {
    fn default() -> Self {
        let endpoint = match option_env!("DYNAMO_ENDPOINT") {
            Some(value) if value.is_empty() || value.eq_ignore_ascii_case("none") => None,
            Some(value) => Some(value),
            None => None,
        };
        Config {
            env: option_env!("ENV").expect("You must set ENV"),
            private_bucket_name: option_env!("PRIVATE_BUCKET_NAME")
                .expect("You must set PRIVATE_BUCKET_NAME"),
            dynamo_endpoint: endpoint,
        }
    }
}

static mut CONFIG: Option<Config> = None;

#[allow(static_mut_refs)]
pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
