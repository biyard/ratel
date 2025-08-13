use bdk::prelude::*;

#[api_model(table = oracles)]
pub struct Oracle {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users, unique)]
    pub user_id: i64,

    #[api_model(action = create, type = INTEGER)]
    pub oracle_type: OracleType,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum OracleType {
    #[default]
    Artist = 1,
    Gallery = 2,
    Collector = 3,
    Auction = 4,
}
