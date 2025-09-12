#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
mod empty_param_tests {
    #![allow(unused)]
    use super::*;
    use std::time::SystemTime;

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::{api_model, ApiModel};

    #[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
    pub struct QueryResponse<T> {
        pub items: Vec<T>,
        pub total_count: i64,
    }

    impl<T> From<(Vec<T>, i64)> for QueryResponse<T> {
        fn from((items, total_count): (Vec<T>, i64)) -> Self {
            QueryResponse { items, total_count }
        }
    }

    #[api_model(base = "/organizations/v2/:org-id/surveys", table = surveys, iter_type=QueryResponse)]
    pub struct SurveyV2 {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(summary, action = create)]
        pub name: String,
        #[api_model(summary, type = INTEGER)]
        pub project_type: ProjectType,
        #[api_model(summary, action = create, type = INTEGER)]
        pub project_area: ProjectArea,
        #[api_model(summary, type = INTEGER)]
        pub status: ProjectStatus,
        #[api_model(summary, action = create)]
        pub started_at: i64,
        #[api_model(summary, action = create)]
        pub ended_at: i64,
        #[api_model(action = create)]
        pub description: String,
        #[api_model(summary, action = create)]
        pub quotes: i64,
        #[api_model(summary, queryable, many_to_one = organizations)]
        pub org_id: i64,
        #[api_model(action = create, type = JSONB, version = v0.1)]
        pub questions: Vec<Question>,
        // #[api_model(summary, one_to_many= responses, aggregator = count)]
        // pub response_count: i64,
        // #[api_model(summary, many_to_many = attrs, foreign_table_name = attributes, foreign_primary_key = attr_id, foreign_reference_key = survey_id)]
        // pub attributes: Vec<Attribute>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel)]
    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
    #[serde(rename_all = "snake_case")]
    pub enum ProjectStatus {
        #[default]
        Ready = 1,
        InProgress = 2,
        Finish = 3,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel)]
    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
    pub enum ProjectType {
        #[default]
        Survey = 1,
        Deliberation = 2,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Copy)]
    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
    pub enum ProjectArea {
        #[default]
        Economy = 1,
        Society = 2,
        Environment = 3,
        Education = 4,
        Culture = 5,
        Labor = 6,
        City = 7,
        Technology = 8,
        Health = 9,
        Politics = 10,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    #[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
    #[serde(rename_all = "snake_case", tag = "answer_type")]
    pub enum Question {
        SingleChoice,
        MultipleChoice,
        ShortAnswer,
        Subjective,
    }
}
