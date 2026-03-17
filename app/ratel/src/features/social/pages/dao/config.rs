use crate::common::CommonConfig;

#[derive(Debug)]
pub struct Config {
    pub common: CommonConfig,
    pub rpc_url: String,
    pub block_explorer_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            common: CommonConfig::default(),
            rpc_url: option_env!("KAIA_ENDPOINT")
                .or(option_env!("KAIA_ENDPOINT"))
                .unwrap_or("https://public-en-kairos.node.kaia.io")
                .to_string(),
            block_explorer_url: option_env!("BLOCK_EXPLORER_URL")
                .or(option_env!("BLOCK_EXPLORER_URL"))
                .unwrap_or("https://kairos.kaiascan.io")
                .to_string(),
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
