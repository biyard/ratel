use crate::models::SpaceCommon;
use crate::utils::reports::NetworkCentralityRow;
use crate::utils::reports::NetworkConfigV1;
use crate::utils::reports::run_network_rows_from_xlsx;
use crate::utils::reports::{LdaConfigV1, TopicRow, run_from_xlsx};
use crate::*;
use axum::{Extension, Json, extract::State};
use std::collections::HashMap;
use std::path::Path;

const XLSX_PATH: &str = "";
// const XLSX_PATH: &str = "/Users/leechanhui/Projects/ratel-copy/ratel/packages/main-api/Questionnaire_answer(12.06.).xlsx";

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    aide::OperationIo,
    schemars::JsonSchema,
)]
pub struct GetSpaceResultResponse {
    pub lda: HashMap<String, Vec<TopicRow>>,
    pub network: HashMap<String, Vec<NetworkCentralityRow>>,
}

pub async fn get_space_result_handler(
    State(AppState { .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    NoApi(_perms): NoApi<Permissions>,
    Extension(_space): Extension<SpaceCommon>,
) -> Result<Json<GetSpaceResultResponse>> {
    // FIXME: fix to query dynamo db
    if !Path::new(XLSX_PATH).exists() {
        return Err(crate::Error::InternalServerError(format!(
            "xlsx not found: {}",
            XLSX_PATH
        )));
    }

    let lda = run_from_xlsx(XLSX_PATH, LdaConfigV1::default())?;
    let network = run_network_rows_from_xlsx(XLSX_PATH, NetworkConfigV1::default())?;

    Ok(Json(GetSpaceResultResponse { lda, network }))
}
