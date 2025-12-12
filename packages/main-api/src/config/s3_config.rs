#[derive(Debug, Clone, Copy)]
pub struct S3Config {
    pub name: &'static str,
    pub asset_dir: &'static str,
    pub expire: u64,
    pub region: &'static str,
}

impl S3Config {
    pub fn get_url(&self, key: &str) -> String {
        format!("https://{}/{}", self.name, key)
    }
}

impl Default for S3Config {
    fn default() -> Self {
        S3Config {
            name: option_env!("BUCKET_NAME").expect("You must set BUCKET_NAME"),
            asset_dir: option_env!("ASSET_DIR").expect("You must set ASSET_DIR"),
            expire: option_env!("BUCKET_EXPIRE")
                .unwrap_or_else(|| {
                    tracing::warn!(
                        "We recommend to set BUCKET_EXPIRE. BUCKET_EXPIRE is not set. Default is 3600."
                    );
                    "3600"
                })
                .parse()
                .unwrap(),
            region: option_env!("S3_REGION").unwrap_or("ap-northeast-2"),
        }
    }
}
