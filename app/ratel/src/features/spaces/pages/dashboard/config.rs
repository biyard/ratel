use crate::common::CommonConfig;

#[derive(Debug, Default)]
pub struct Config {
    pub common: CommonConfig,
}

impl Config {
    #[cfg(feature = "server")]
    pub fn dynamodb(&self) -> &aws_sdk_dynamodb::Client {
        self.common.dynamodb()
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
