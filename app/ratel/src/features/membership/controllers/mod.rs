mod get_billing_info;
mod update_billing_card;
mod change_membership;
mod change_team_membership;
mod get_membership;
mod portone_hook;
mod get_team_billing_info;
mod get_team_membership;
mod history;
mod identify;
mod identify_team;
mod update_team_billing_card;
mod team_history;

use super::*;
pub use get_billing_info::*;
pub use update_billing_card::*;
pub use change_membership::*;
pub use change_team_membership::*;
use crate::common::Error;
pub use get_membership::*;
pub use portone_hook::*;
pub use get_team_billing_info::*;
pub use get_team_membership::*;
pub use history::*;
pub use identify::*;
pub use identify_team::*;
pub use update_team_billing_card::*;
pub use team_history::*;

#[cfg(feature = "server")]
pub(crate) fn mask_card_number(card_number: &str) -> String {
    let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 4 {
        return "****".to_string();
    }
    let last4 = &digits[digits.len() - 4..];
    format!("****-****-****-{last4}")
}

#[cfg(feature = "server")]
pub(crate) fn normalize_error(err: Error) -> Error {
    match err {
        Error::Aws(e) => Error::Unknown(format!("AWS error: {}", e)),
        Error::Session(e) => Error::Unknown(format!("Session error: {}", e)),
        other => other,
    }
}

#[cfg(not(feature = "server"))]
pub(crate) fn normalize_error(err: Error) -> Error {
    err
}
