use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomWallet {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webapp_link: Option<String>,
}

#[derive(Debug, Clone)]
pub struct WalletConnectConfig {
    pub project_id: String,
    pub app_name: String,
    pub app_description: String,
    pub app_url: String,
}

impl Default for WalletConnectConfig {
    fn default() -> Self {
        WalletConnectConfig {
            project_id: option_env!("WALLETCONNECT_PROJECT_ID")
                .unwrap_or_else(|| {
                    warn!("WALLETCONNECT_PROJECT_ID is not set. WalletConnect will not work.");
                    ""
                })
                .to_string(),
            app_name: "Ratel".to_string(),
            app_description: "Decentralized Legislative Platform".to_string(),
            app_url: "".to_string(),
        }
    }
}

impl WalletConnectConfig {
    pub fn custom_wallets(&self) -> Vec<CustomWallet> {
        vec![CustomWallet {
            id: "kaia".to_string(),
            name: "Kaia Wallet".to_string(),
            homepage: Some("https://www.kaiawallet.io".to_string()),
            image_url: Some(
                "https://www.kaiawallet.io/favicon.ico".to_string(),
            ),
            mobile_link: Some("kaiawallet://".to_string()),
            desktop_link: None,
            webapp_link: Some("https://app.kaiawallet.io".to_string()),
        }]
    }
}
