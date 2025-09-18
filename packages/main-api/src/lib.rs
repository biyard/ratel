pub type Result<T> = dto::Result<T>;
pub type Error = dto::Error;
pub type Error2 = crate::error::Error;

pub mod error;
pub mod controllers {
    pub mod mcp;
    pub mod v1;
    pub mod v2 {
        pub mod users {
            pub mod connect_telegram;
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
            pub mod extract_medical_info;
            pub mod extract_passport_info;
            pub mod upload_private_image;
        }
        pub mod conversations {
            pub mod add_conversations;
            pub mod get_conversation_by_id;
            pub mod get_conversations;

            pub mod messages {
                pub mod add_messages;
                pub mod clear_message;
                pub mod get_messages;
                pub mod poll_messages;

                #[cfg(test)]
                pub mod tests;
            }

            #[cfg(test)]
            pub mod tests;
        }
        pub mod oauth {
            pub mod approve;
            pub mod authorize;
            pub mod oauth_authorization_server;
            pub mod register;
            pub mod token;
        }

        pub mod posts {
            pub mod get_post;
            pub mod list_posts;
            pub mod update_post;
        }

        pub mod themes {
            pub mod change_theme;
        }

        pub mod telegram {
            pub mod get_telegram_info;
            pub mod verify_telegram_raw;
        }

        pub mod binances {
            pub mod binance_webhook;
            pub mod create_subscription;
            pub mod unsubscribe;
        }
    }
    pub mod v3 {
        // pub mod users {
        //     pub mod get_user_info;
        // }

        pub mod auth {
            pub mod email_login;
            pub mod signup;
            // pub mod verification {
            //     pub mod send_code;
            //     pub mod verify_code;
            // }
        }
    }
    pub mod m2 {
        pub mod noncelab {
            pub mod users {
                pub mod register_users;
            }
        }
        pub mod migration {
            pub mod postgres_to_dynamodb;
        }
        pub mod binances {
            pub mod get_merchant_balance;
        }
    }
    pub mod well_known {
        pub mod get_did_document;
    }
    pub mod wg {
        pub mod get_home;
    }
}

pub mod api_main;
pub mod config;
pub mod etl;
pub mod models;
pub mod route;
pub mod security;
pub mod types;
pub mod utils;

pub use bdk::prelude::*;
pub use dto::*;

#[cfg(test)]
pub mod tests;
