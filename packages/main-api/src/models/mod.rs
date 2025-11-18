pub mod folder_type {
    pub mod folder_type;
}

pub mod email_template {
    pub mod email_template;
}

pub mod oauth {
    pub mod code_challenge;
    pub mod grant_type;
    pub mod response_type;
    pub mod scope;
}

pub mod dynamo_tables;

pub use dynamo_tables::*;
