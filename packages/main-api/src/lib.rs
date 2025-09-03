pub mod controllers {
    pub mod m1;
    pub mod mcp;
    pub mod v1;
    pub mod v2 {
        pub mod users {
            pub mod find_user;
            pub mod logout;
        }
        pub mod industries {
            pub mod industry;
            pub mod select_topic;
        }
        pub mod networks {
            pub mod accept_invitation;
            pub mod accept_suggestion;
            pub mod list_networks;
            pub mod reject_invitation;
            pub mod reject_suggestion;
        }
        pub mod connections {
            pub mod follow;
            pub mod network;
            pub mod search;
        }
        pub mod notifications {
            pub mod get_notifications;
            pub mod mark_all_read;
        }
        pub mod telegram {
            pub mod subscribe;
        }

        pub mod dashboards {
            pub mod get_dashboard;
        }

        pub mod bookmarks {
            pub mod add_bookmark;
            pub mod list_bookmarks;
            pub mod remove_bookmark;
        }

        // Spaces APIs
        pub mod spaces {
            pub mod delete_space;
            pub mod get_my_space;
        }

        pub mod dagits {
            pub mod add_oracle;
            pub mod get_dagit;

            pub mod artworks {
                pub mod create_artwork;
                pub mod get_artwork_certificate;
                pub mod get_artwork_detail;
            }

            pub mod consensus {
                pub mod create_consensus;
                pub mod vote;
            }
        }

        pub mod oracles {
            pub mod create_oracle;
        }

        pub mod documents {
            pub mod extract_passport_info;
            pub mod upload_private_image;
        }

        pub mod oauth {
            pub mod approve;
            pub mod authorize;
            pub mod oauth_authorization_server;
            pub mod register;
            pub mod token;
        }
    }
    pub mod m2 {
        pub mod noncelab {
            pub mod users {
                pub mod register_users;
            }
        }
    }
    pub mod well_known {
        pub mod get_did_document;
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
