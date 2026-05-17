mod types;
mod invoke;
mod commands;

pub use types::*;
pub use commands::*;
pub use invoke::*;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[macro_export]
macro_rules! define_invoke_tauri {
    ($fn:ident, $method:expr) => {
        #[cfg(feature = "web")]
        pub async fn $fn() -> crate::Result<()> {
            crate::invoke($method, ()).await
        }
    };

    ($fn:ident, $method:expr, $args:ty) => {
        #[cfg(feature = "web")]
        pub async fn $fn(args: &$args) -> crate::Result<()> {
            crate::invoke($method, args).await
        }
    };

    ($fn:ident, $method:expr, res: $res:ty) => {
        #[cfg(feature = "web")]
        pub async fn $fn() -> crate::Result<$res> {
            crate::invoke($method, ()).await
        }
    };

    ($fn:ident, $method:expr, $args:ty, $res:ty) => {
        #[cfg(feature = "web")]
        pub async fn $fn(args: &$args) -> crate::Result<$res> {
            crate::invoke($method, args).await
        }
    };
}
