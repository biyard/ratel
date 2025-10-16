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

    pub mod get_deliberation_common;
    pub mod get_deliberation_deliberation;
    pub mod get_deliberation_poll;
    pub mod get_deliberation_recommendation;
    pub mod get_deliberation_summary;

    pub mod update_deliberation_deliberation;
    pub mod update_deliberation_poll;
    pub mod update_deliberation_recommendation;
    pub mod update_deliberation_summary;

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

pub mod polls;

pub use create_space::*;
pub use delete_space::*;
pub use polls::*;
pub use update_space::*;
