use ethers::types::U256;
use x402_axum::layer::X402Error;
use x402_rs::types::TokenAmount;

use super::*;
use x402_axum::layer::DynamicPriceCallback;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct GetSpaceReportResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn get_space_report_handler(
    State(AppState { .. }): State<AppState>,
) -> Result<Json<GetSpaceReportResponse>> {
    // TODO: Implement the handler logic here

    let response = GetSpaceReportResponse {
        status: "Space report generated successfully".to_string(),
    };

    Ok(Json(response))
}

async fn get_space_report_price(
    _headers: &http::HeaderMap,
    _uri: &http::Uri,
    _base_url: &url::Url,
) -> std::result::Result<TokenAmount, X402Error> {
    // TODO: implement report pricing per a space
    Ok(TokenAmount::from(10u128))
}

pub fn get_usdt_price_callback() -> Box<DynamicPriceCallback> {
    let callback: Box<DynamicPriceCallback> = Box::new(move |headers, uri, base_url| {
        Box::pin(async move { get_space_report_price(headers, uri, base_url).await })
    });

    callback
}
