use crate::common::services::icp::IcpSamplingService;
use dioxus::fullstack::Lazy;

pub static ICP_SAMPLING_SERVICE: Lazy<IcpSamplingService> = Lazy::new(|| async move {
    let ic_url = option_env!("IC_URL").unwrap_or_else(|| {
        tracing::warn!("IC_URL not set, using local default (http://127.0.0.1:4943)");
        "http://127.0.0.1:4943"
    });
    let canister_id = option_env!("SAMPLING_CANISTER_ID").unwrap_or_else(|| {
        tracing::warn!("SAMPLING_CANISTER_ID not set");
        ""
    });
    dioxus::Ok(IcpSamplingService::new(ic_url, canister_id).await?)
    // let config = BiyardConfig::default();
    // dioxus::Ok(BiyardService::new(
    //     config.api_secret.to_string(),
    //     config.project_id.to_string(),
    //     config.base_url.to_string(),
    // ))
});
