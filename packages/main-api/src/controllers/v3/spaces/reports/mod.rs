pub mod create_report;
pub mod get_pricing_challenge;
pub mod get_report;
mod get_space_report;
pub mod publish_report;
pub mod set_pricing;
pub mod update_report;

pub use create_report::*;
pub use get_pricing_challenge::*;
pub use get_report::*;
pub use publish_report::*;
pub use set_pricing::*;
pub use update_report::*;

#[cfg(test)]
pub mod tests;

use crate::*;

use x402_axum::{IntoPriceTag, X402Middleware};
use x402_rs::network::{Network, USDCDeployment};

pub fn route() -> Router<AppState> {
    let x402_config = config::get().x402;
    let x402 = X402Middleware::try_from(x402_config.facilitator_url)
        .expect("incorrect facilitator_url")
        .with_base_url(url::Url::parse("http://localhost:3000/").unwrap());
    let usdc = USDCDeployment::by_network(Network::BaseSepolia);

    Router::new()
        .route(
            "/",
            get(get_space_report::get_space_report_handler).layer(
                x402.with_price_tag(usdc.amount("0.025").pay_to(x402_config.address()).unwrap())
                    .with_dynamic_price(get_space_report::get_usdt_price_callback()),
            ),
        )
        // POST to create a new report
        .route("/", post(create_report_handler))
        // GET and PATCH for author to view/edit their draft report
        // (separate from x402-protected consumer endpoint at parent level)
        .route(
            "/draft",
            get(get_report_handler).patch(update_report_handler),
        )
        .route("/pricing/challenge", post(get_pricing_challenge_handler))
        .route("/pricing", post(set_pricing_handler))
        .route("/publish", post(publish_report_handler))
}
