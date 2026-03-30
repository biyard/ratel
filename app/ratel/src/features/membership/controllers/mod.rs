mod change_membership;
mod change_team_membership;
mod get_membership;
mod get_team_membership;
mod history;
mod identify;
mod identify_team;

use super::*;
pub use change_membership::*;
pub use change_team_membership::*;
use crate::common::Error;
pub use get_membership::*;
pub use get_team_membership::*;
pub use history::*;
pub use identify::*;
pub use identify_team::*;

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
