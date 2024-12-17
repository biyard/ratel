#[derive(Debug)]
pub struct Config {
    pub env: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("env").expect("You must set ENV").to_string(),
        }
    }
}

static mut CONFIG: Option<Config> = None;

pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
