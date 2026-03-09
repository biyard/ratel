use crate::common::CommonConfig;

#[derive(Debug)]
pub struct Config {
    pub common: CommonConfig,
}

#[cfg(feature = "server")]
impl Config {
    pub fn dynamodb(&self) -> &crate::common::aws_sdk_dynamodb::Client {
        self.common.dynamodb()
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
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
