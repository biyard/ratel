use crate::common::CommonConfig;
pub use crate::common::PortoneConfig;

#[derive(Debug, Default)]
pub struct Config {
    pub common: CommonConfig,
    pub portone: PortoneConfig,
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
