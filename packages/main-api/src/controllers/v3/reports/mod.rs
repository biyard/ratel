pub mod report_content;

pub use report_content::*;

#[cfg(test)]
pub mod tests;

use crate::*;

pub fn route() -> Result<Router<AppState>> {
    Ok(Router::new().route("/", post(report_content_handler)))
}
