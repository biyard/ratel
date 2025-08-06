pub mod controllers {
    pub mod m1;
    pub mod mcp;
    pub mod v1;
    pub mod v2 {
        pub mod users {
            pub mod logout;
        }
        pub mod telegram {
            pub mod subscribe;
        }
    }
    pub mod m2 {
        pub mod noncelab {
            pub mod users {
                pub mod register_users;
            }
        }
    }
}

pub mod api_main;
pub mod config;
pub mod models;
pub mod route;
pub mod security;
pub mod utils;

pub use bdk::prelude::*;
pub use dto::*;

#[cfg(test)]
pub mod tests;
