pub mod controllers {
    pub mod m1;
    pub mod mcp;
    pub mod v1;
    // pub mod v2 {
    //     pub mod users {
    //         pub mod find_user;
    //         pub mod logout;
    //     }
    //     pub mod industries {
    //         pub mod industry;
    //         pub mod select_topic;
    //     }
    //     pub mod networks {
    //         pub mod follow;
    //         pub mod network;
    //         pub mod search;
    //     }
    //     pub mod telegram {
    //         pub mod subscribe;
    //     }

    //     pub mod dagits {
    //         pub mod add_oracle;
    //         pub mod get_dagit;

    //         pub mod artworks {
    //             pub mod create_artwork;
    //             pub mod get_artwork_detail;
    //         }

    //         pub mod consensus {
    //             pub mod create_consensus;
    //             pub mod vote;
    //         }
    //     }
    //     pub mod oracles {
    //         pub mod create_oracle;
    //     }
    // }
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
