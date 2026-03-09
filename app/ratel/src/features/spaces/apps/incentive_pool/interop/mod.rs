use crate::features::spaces::apps::incentive_pool::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeploySpaceIncentiveRequest {
    #[serde(default)]
    pub admins: Vec<String>,
    pub incentive_recipient_count: i64,
    pub ranking_bps: i64,
    pub mode: i64,
    pub env: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeploySpaceIncentiveResponse {
    pub incentive_address: String,
    pub deploy_block: i64,
    pub transaction_hash: String,
    pub admin_address: String,
}

#[cfg(not(feature = "server"))]
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsDeployRequest {
    admins: Vec<String>,
    incentiveRecipientCount: i64,
    rankingBps: i64,
    mode: i64,
    env: String,
}

#[cfg(not(feature = "server"))]
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct JsDeployResponse {
    incentiveAddress: String,
    deployBlock: i64,
    transactionHash: String,
    adminAddress: String,
}

#[cfg(not(feature = "server"))]
use crate::common::wasm_bindgen::prelude::*;
#[cfg(not(feature = "server"))]
use crate::common::wasm_bindgen_futures::JsFuture;
#[cfg(not(feature = "server"))]
use crate::common::web_sys::js_sys::{Promise, Reflect, JSON};

#[cfg(not(feature = "server"))]
#[wasm_bindgen(js_namespace = ["window", "ratel", "space_incentive_pool"])]
extern "C" {
    #[wasm_bindgen(js_name = deploySpaceIncentive, catch)]
    fn deploy_space_incentive_js(req: &JsValue) -> std::result::Result<Promise, JsValue>;

    #[wasm_bindgen(js_name = copyText, catch)]
    fn copy_text_js(text: &str) -> std::result::Result<Promise, JsValue>;
}

#[cfg(not(feature = "server"))]
pub async fn deploy_space_incentive(
    req: DeploySpaceIncentiveRequest,
) -> Result<DeploySpaceIncentiveResponse> {
    let js_req = JsDeployRequest {
        admins: req.admins,
        incentiveRecipientCount: req.incentive_recipient_count,
        rankingBps: req.ranking_bps,
        mode: req.mode,
        env: req.env,
    };

    let js_req = crate::common::serde_wasm_bindgen::to_value(&js_req)
        .map_err(|err| Error::Unknown(format!("failed to serialize deploy request: {err}")))?;

    let promise =
        deploy_space_incentive_js(&js_req).map_err(|err| Error::Unknown(format_js_error(err)))?;
    let value = JsFuture::from(promise)
        .await
        .map_err(|err| Error::Unknown(format_js_error(err)))?;

    let res: JsDeployResponse = crate::common::serde_wasm_bindgen::from_value(value)
        .map_err(|err| Error::Unknown(format!("invalid deploy response: {err}")))?;

    Ok(DeploySpaceIncentiveResponse {
        incentive_address: res.incentiveAddress,
        deploy_block: res.deployBlock,
        transaction_hash: res.transactionHash,
        admin_address: res.adminAddress,
    })
}

#[cfg(feature = "server")]
pub async fn deploy_space_incentive(
    _req: DeploySpaceIncentiveRequest,
) -> Result<DeploySpaceIncentiveResponse> {
    Err(Error::NotSupported(
        "deploy is only available on web".to_string(),
    ))
}

#[cfg(not(feature = "server"))]
pub async fn copy_text(text: String) -> Result<()> {
    let promise = copy_text_js(&text).map_err(|err| Error::Unknown(format_js_error(err)))?;
    JsFuture::from(promise)
        .await
        .map_err(|err| Error::Unknown(format_js_error(err)))?;
    Ok(())
}

#[cfg(feature = "server")]
pub async fn copy_text(_text: String) -> Result<()> {
    Err(Error::NotSupported(
        "clipboard is only available on web".to_string(),
    ))
}

#[cfg(not(feature = "server"))]
pub fn open_url(url: &str) {
    let _ = crate::common::web_sys::window().and_then(|window| window.open_with_url(url).ok());
}

#[cfg(feature = "server")]
pub fn open_url(_url: &str) {}

#[cfg(not(feature = "server"))]
fn format_js_error(err: JsValue) -> String {
    if let Some(msg) = err.as_string() {
        return msg;
    }

    if err.is_object() {
        if let Ok(message) = Reflect::get(&err, &JsValue::from_str("message")) {
            if let Some(msg) = message.as_string() {
                return msg;
            }
        }
    }

    if let Ok(json) = JSON::stringify(&err) {
        if let Some(msg) = json.as_string() {
            return msg;
        }
    }

    "Unknown error".to_string()
}
