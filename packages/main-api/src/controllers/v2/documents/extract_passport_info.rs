use crate::{
    config,
    utils::{
        aws::{BedrockClient, RekognitionClient, S3Client, TextractClient},
        users::extract_user_id,
    },
};
use bdk::prelude::*;
use dto::{
    Error, JsonSchema, Result, aide,
    by_axum::{
        auth::Authorization,
        axum::{Extension, Json, extract::State},
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct PassportRequest {
    #[schemars(description = "S3 Object Key of the passport image within the s3 bucket")]
    pub key: String,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct PassportResponse {
    #[schemars(description = "Passport verification result")]
    pub result: PassportInfo,
}

#[derive(
    Default, Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct PassportInfo {
    pub first_name: String,
    pub last_name: String,
    #[serde(deserialize_with = "date_format::deserialize")]
    pub birth_date: i64,
    pub nationality: String,
    #[serde(deserialize_with = "date_format::deserialize")]
    pub expiration_date: i64,
    pub gender: Gender,
}

mod date_format {
    use chrono::{NaiveDate, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y/%m/%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        let naive_datetime = dt.and_hms_opt(0, 0, 0).unwrap();
        Ok(Utc.from_utc_datetime(&naive_datetime).timestamp())
    }
}

#[derive(
    Debug, Default, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub enum Gender {
    Male,
    Female,
    #[default]
    Other,
}

impl From<&str> for Gender {
    fn from(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "male" => Gender::Male,
            "female" => Gender::Female,
            "other" => Gender::Other,
            _ => Gender::Other,
        }
    }
}

#[derive(Clone)]
pub struct PassportHandlerState {
    pub pool: PgPool,
    pub bedrock_client: BedrockClient,
    pub rek_client: RekognitionClient,
    pub textract_client: TextractClient,
    pub s3_client: S3Client,
}

pub async fn extract_passport_info_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(state): State<PassportHandlerState>,
    Json(req): Json<PassportRequest>,
) -> Result<Json<PassportResponse>> {
    extract_user_id(&state.pool, auth).await?;
    let passport_info = worker(&state, &req).await;
    if let Err(e) = state.s3_client.delete_object(&req.key).await {
        tracing::error!("Failed to delete S3 object {}: {:?}", &req.key, e);
    }
    Ok(Json(PassportResponse {
        result: passport_info?,
    }))
}

async fn worker(state: &PassportHandlerState, req: &PassportRequest) -> Result<PassportInfo> {
    /*
    1. Detect if the image is a passport using AWS Rekognition
    Rekognition `detect labels` API Price:
        $0.0012 per image

    2. Extract text from the passport image using AWS Textract
    Textract `detect_document_text` API Price:
        $0.0015 per image

    3. Use AWS Bedrock to parse the extracted text and extract passport fields
    Bedrock NOVA.micro Model Price:
        $0.000041 per 1K input tokens
        $0.000164 per 1K output tokens


    Estimated: $0.00003444
        (per request 600 input tokens and 60 output tokens used)
        ($0.000041 * 0.6 + $0.000164 * 0.06 = $0.0000246 + $0.00000984)

    **Total estimated cost per request: $0.00283444 (~ 3.71 KRW)**
    */

    let image =
        RekognitionClient::get_image_from_s3_object(config::get().private_bucket_name, &req.key);

    let rek_output = state
        .rek_client
        .detect_labels_from_image(image, Some(10), Some(80.0));
    let is_passport = rek_output
        .await?
        .iter()
        .any(|label| label.name().is_some_and(|v| v == "Passport"));

    if !is_passport {
        return Err(Error::PassportVerificationFailed(
            "Image is not a passport".to_string(),
        ));
    }

    let document =
        TextractClient::get_document_from_s3_object(config::get().private_bucket_name, &req.key);
    let detected_text = state.textract_client.detect_document_text(document).await?;
    let merged_text = detected_text.join("\n");

    let prompt = format!(
        r#"
        You are an expert data extraction AI.
        From the passport OCR text below, extract the `first_name`, `last_name`, `nationality`, `birth_date`, `sex`, and `expiration_date`. Pay close attention to the Machine Readable Zone (MRZ).

        Your response **must** be a single, raw JSON object. Use `null` for missing fields. Add no explanations or markdown.

        **-- Example --**
        **Input Text:**
        2 PASSPORT
REPUBLIC OF KOREA
Type 2115/ Country code
1 Passport No.
PM
KOR
M555Y1234
Surname
HONG
Given names
GILDONG
2020
TOTAL
Date of birth
/ Sex
01 /JAN 2000
M
Nationality
18 28/ Authority
REPUBLIC OF KOREA
MINISTRY OF FOREIGN AFFAIRS
WHI Date of issue
Date of expiry
01 /JAN 2020
01 /JAN 2030
        **JSON Output:**
        `{{"first_name":"GILDONG", "last_name":"HONG", "nationality":"ROK","birth_date":"2000/01/01","gender":"Male","expiration_date":"2030/01/01"}}`

        **-- Task --**
        **Input Text:**
        `{}`
        **JSON Output:**
        "#,
        merged_text
    );

    let text = state.bedrock_client.send_message(prompt).await?;
    let json_str = extract_json(&text).ok_or_else(|| {
        tracing::error!("Failed to extract JSON from Bedrock response: {}", text);
        Error::PassportVerificationFailed("Failed to extract JSON from response".to_string())
    })?;
    let passport_info: PassportInfo = serde_json::from_str(&json_str).map_err(|e| {
        tracing::error!("Failed to parse JSON string: {}, error: {:?}", json_str, e);
        Error::PassportVerificationFailed("Failed to parse JSON from response".to_string())
    })?;

    Ok(passport_info)
}

fn extract_json(text: &str) -> Option<&str> {
    if let Some(start) = text.find('{') {
        if let Some(end) = text.rfind('}') {
            if end > start {
                return Some(&text[start..=end]);
            }
        }
    }
    None
}
