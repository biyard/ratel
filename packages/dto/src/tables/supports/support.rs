use bdk::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Default, by_macros::ApiModel, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum Need {
    #[default]
    GeneralInquiry = 1,
    TechnicalSupport = 2,
    PartnershipCollaboration = 3,
    InvestmentFunding = 4,
    RegulatoryLegalConcerns = 5,
    FeedbackSuggestions = 6,
}

#[derive(validator::Validate)]
// TODO(api): POST /v1/supports (submit)
#[api_model(base = "/v1/supports", table = supports)]
pub struct Support {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = submit)]
    pub first_name: String,
    #[api_model(action = submit)]
    pub last_name: String,

    #[validate(email)]
    #[api_model(action = submit)]
    pub email: String,
    #[api_model(action = submit)]
    pub company_name: String,
    #[api_model(type = INTEGER, action = submit)]
    pub needs: Need,
    #[api_model(action = submit)]
    pub help: String,
}
