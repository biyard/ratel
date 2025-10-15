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

pub mod spaces {
    pub mod create_space;
    pub mod delete_space;
    pub mod update_space;

    #[cfg(test)]
    pub mod tests;

    pub mod deliberations {
        pub mod create_deliberation;

        pub mod delete_deliberation;
        pub mod get_deliberation;
        pub mod posting_deliberation;

        pub mod discussions {
            pub mod create_discussion;
            pub mod end_recording;
            pub mod exit_meeting;
            pub mod get_meeting;
            pub mod participant_meeting;
            pub mod start_meeting;
            pub mod start_recording;

            pub mod get_discussion;

            #[cfg(not(feature = "no-secret"))]
            #[cfg(test)]
            pub mod tests;
        }

        pub mod responses {
            pub mod create_response_answer;
            pub mod get_response_answer;

            #[cfg(test)]
            pub mod tests;
        }
        #[cfg(test)]
        pub mod tests;
        pub mod update_deliberation;
    }

    pub mod poll {
        // pub mod create_poll_space;
        pub mod get_poll_space;
        pub mod get_survey_summary;
        pub mod respond_poll_space;
        pub mod update_poll_space;

        #[cfg(test)]
        pub mod tests;
    }
}

pub mod teams {
    pub mod create_team;
    pub mod delete_team;
    pub mod find_team;
    pub mod get_team;
    pub mod list_members;
    pub mod update_team;

    pub mod dto;
    pub mod get_permissions;
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
