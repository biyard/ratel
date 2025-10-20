pub mod networks;

pub mod promotions {
    pub mod get_top_promotion;
}
pub mod me {
    pub mod get_info;
    pub mod update_user;

    pub mod list_my_drafts;
    pub mod list_my_posts;
    #[cfg(test)]
    pub mod tests;
}
pub mod users {
    pub mod find_user;

    #[cfg(test)]
    pub mod tests;
}

pub mod assets {
    pub mod complete_multipart_upload;
    pub mod get_put_multi_object_uri;
    pub mod get_put_object_uri;
}

pub mod auth {
    pub mod health;
    pub mod login;
    pub mod logout;
    pub mod signup;

    #[cfg(test)]
    pub mod tests;

    pub mod verification {
        pub mod send_code;
        pub mod verify_code;

        #[cfg(test)]
        pub mod tests;
    }
}

pub mod spaces;

pub mod teams {
    pub mod create_team;
    pub mod delete_team;
    pub mod find_team;
    pub mod get_team;
    pub mod list_members;
    pub mod list_team_posts;
    pub mod update_team;

    pub mod dto;
    #[cfg(test)]
    pub mod tests;

    pub mod groups {
        pub mod add_member;
        pub mod create_group;
        pub mod delete_group;
        pub mod remove_member;
        pub mod update_group;

        #[cfg(test)]
        pub mod tests;
    }
}

pub mod posts;
