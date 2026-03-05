mod change_membership;
mod get_membership;
mod history;
mod identify;

pub use change_membership::*;
use common::Error;
pub use get_membership::*;
pub use history::*;
pub use identify::*;

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
