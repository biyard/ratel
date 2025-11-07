use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct CreateAttiributeCodeRequest {
    pub birth_date: Option<String>, // YYYYMMDD
    pub gender: Option<Gender>,
    pub university: Option<String>,
}

pub async fn create_attiribute_code_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(CreateAttiributeCodeRequest {
        birth_date,
        gender,
        university,
    }): Json<CreateAttiributeCodeRequest>,
) -> Result<Json<AttributeCode>> {
    let mut ac = AttributeCode::new();

    if let Some(birth_date) = birth_date {
        ac.birth_date = Some(birth_date);
    }

    if let Some(gender) = gender {
        ac.gender = Some(gender);
    }

    if let Some(university) = university {
        ac.university = Some(university);
    }

    ac.create(&dynamo.client).await?;

    Ok(Json(ac))
}
