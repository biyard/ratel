use aws_sdk_textract::types::Document;
use bdk::prelude::*;
use dto::{
    Error, JsonSchema, Result, aide,
    by_axum::axum::{Json, extract::State},
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::utils::aws::{BedrockClient, RekognitionClient, TextractClient};

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct PassportRequest {
    #[schemars(description = "Image byte of the passport(Max Size : 10MB)")]
    pub image_byte: Vec<u8>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct PassportResponse {
    #[schemars(description = "Passport verification result")]
    pub result: Option<PassportInfo>,
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub enum Gender {
    Male,
    Female,
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
}

pub async fn extract_passport_info_handler(
    State(state): State<PassportHandlerState>,
    Json(req): Json<PassportRequest>,
) -> Result<Json<PassportResponse>> {
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
    let rek_output =
        state
            .rek_client
            .detect_labels_from_image(req.image_byte.clone(), Some(10), Some(80.0));
    let is_passport = rek_output
        .await?
        .iter()
        .any(|label| label.name().is_some_and(|v| v == "Passport"));

    if !is_passport {
        return Err(Error::PassportVerificationFailed(
            "Image is not a passport".to_string(),
        ));
    }

    let document = Document::builder()
        .bytes(req.image_byte.clone().into())
        .build();
    let detected_text = state.textract_client.detect_labels(document).await?;
    let merged_text = detected_text.join("\n");
    tracing::debug!("Detected text: {}", merged_text);

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
M544Y8844
Surname
HONG
Given names
GILDONG
2020
TOTAL
Date of birth
/ Sex
11 /JAN 1996
M
Nationality
18 28/ Authority
REPUBLIC OF KOREA
MINISTRY OF FOREIGN AFFAIRS
WHI Date of issue
Date of expiry
04 /AUG 2023
04 /AUG 2033
PMKORCHO<<GEONUNG
M544Y88443K0R9601112M33080403713887V25733710
        **JSON Output:**
        `{{"first_name":"GILDONG", "last_name":"HONG", "nationality":"USA","birth_date":"1996/01/11","gender":"Male","expiration_date":"2033/08/04"}}`

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
    tracing::debug!("Extracted JSON string: {}", json_str);
    let passport_info: std::result::Result<PassportInfo, _> = serde_json::from_str(&json_str);
    if let Err(err) = passport_info {
        tracing::error!("Failed to parse passport info: {:?}", err);
        return Err(Error::PassportVerificationFailed(
            "Failed to parse passport info".to_string(),
        ));
    }

    tracing::debug!("Parsed passport info: {:?}", passport_info);

    Ok(Json(PassportResponse {
        result: Some(passport_info.unwrap()),
    }))
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
