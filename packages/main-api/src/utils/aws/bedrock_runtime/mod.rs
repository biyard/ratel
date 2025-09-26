#[cfg(not(feature = "no-secret"))]
mod aws;

#[cfg(not(feature = "no-secret"))]
pub use aws::*;


#[cfg(feature = "no-secret")]
mod noop;

#[cfg(feature = "no-secret")]
pub use noop::*;
