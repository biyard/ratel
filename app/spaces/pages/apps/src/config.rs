use common::CommonConfig;

#[derive(Debug)]
pub struct Config {
    pub common: CommonConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            common: CommonConfig::default(),
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
        CONFIG.as_ref().unwrap()
    }
}
