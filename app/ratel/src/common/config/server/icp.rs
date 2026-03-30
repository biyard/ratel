use crate::common::services::icp::CanisterService;
use dioxus::fullstack::Lazy;
use dioxus::prelude::ServerFnError;

pub static CANISTER_SERVICE: Lazy<CanisterService> = Lazy::new(|| async move {
    let ic_url = std::env::var("IC_URL").unwrap_or_else(|_| {
        let fallback = option_env!("IC_URL").unwrap_or("http://127.0.0.1:4943");
        tracing::warn!("IC_URL not set at runtime, using: {}", fallback);
        fallback.to_string()
    });
    let canister_id = std::env::var("RATEL_CANISTER_ID").unwrap_or_else(|_| {
        let fallback = option_env!("RATEL_CANISTER_ID").unwrap_or("");
        tracing::warn!("RATEL_CANISTER_ID not set at runtime, using: {}", fallback);
        fallback.to_string()
    });
    let identity = if let Ok(path) = std::env::var("ICP_IDENTITY_PEM_PATH") {
        Some(
            ic_agent::identity::Secp256k1Identity::from_pem_file(path.trim())
                .map_err(|e| ServerFnError::new(format!("IC identity load error: {}", e)))?,
        )
    } else if let Ok(pem) = std::env::var("ICP_IDENTITY_PEM") {
        let normalized = pem.replace("\\n", "\n");
        Some(
            ic_agent::identity::Secp256k1Identity::from_pem(normalized.as_bytes())
                .map_err(|e| ServerFnError::new(format!("IC identity parse error: {}", e)))?,
        )
    } else {
        None
    };

    dioxus::Ok(CanisterService::new(&ic_url, &canister_id, identity).await?)
});
