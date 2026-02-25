use common::CommonConfig;

#[derive(Debug)]
pub struct Config {
    pub common: CommonConfig,
    #[cfg(feature = "server")]
    pub s3: S3Config,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            common: CommonConfig::default(),
            #[cfg(feature = "server")]
            s3: S3Config::default(),
        }
    }
}

#[cfg(feature = "server")]
#[derive(Debug, Clone, Copy)]
pub struct S3Config {
    pub name: &'static str,
    pub asset_dir: &'static str,
    pub expire: u64,
    pub region: &'static str,
}

#[cfg(feature = "server")]
impl S3Config {
    pub fn get_url(&self, key: &str) -> String {
        format!("https://{}/{}", self.name, key)
    }
}

#[cfg(feature = "server")]
impl Default for S3Config {
    fn default() -> Self {
        let bucket_name = std::env::var("BUCKET_NAME")
            .ok()
            .or_else(|| option_env!("BUCKET_NAME").map(|v| v.to_string()))
            .unwrap_or_else(|| "metadata.ratel.foundation".to_string());
        let asset_dir = std::env::var("ASSET_DIR")
            .ok()
            .or_else(|| option_env!("ASSET_DIR").map(|v| v.to_string()))
            .unwrap_or_else(|| "metadata".to_string());
        let expire = std::env::var("BUCKET_EXPIRE")
            .ok()
            .or_else(|| option_env!("BUCKET_EXPIRE").map(|v| v.to_string()))
            .unwrap_or_else(|| "3600".to_string())
            .parse()
            .unwrap_or(3600);
        let region = std::env::var("S3_REGION")
            .ok()
            .or_else(|| option_env!("S3_REGION").map(|v| v.to_string()))
            .unwrap_or_else(|| "ap-northeast-2".to_string());

        S3Config {
            name: Box::leak(bucket_name.into_boxed_str()),
            asset_dir: Box::leak(asset_dir.into_boxed_str()),
            expire,
            region: Box::leak(region.into_boxed_str()),
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
