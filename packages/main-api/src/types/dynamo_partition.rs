use bdk::prelude::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, DynamoEnum)]
#[dynamo_enum(error = "crate::Error2")]
pub enum Partition {
    #[default]
    None,

    User(String),
    Email(String),
    Feed(String),

    // Spaces
    Space(String),
    DeliberationSpace(String),
    PollSpace(String),
    SurveySpace(String),

    Team(String),

    // DID/VC Partitions
    Credential(String),
    CredentialOffer(String),
    CredentialTemplate(String),
    StatusList(String),
    OAuthToken(String),
    OAuthClient(String),
    IssuerConfig(String),
    AuditLog(String),
    Presentation(String),
}
