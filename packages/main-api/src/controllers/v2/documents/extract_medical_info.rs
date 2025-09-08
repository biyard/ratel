use crate::utils::{
    aws::{BedrockClient, BedrockModel, S3Client, S3ContentType},
    parse_json::parse_json,
    users::extract_user_id,
};
use aws_sdk_bedrockruntime::types::{ContentBlock, ImageBlock, ImageFormat};
use aws_sdk_s3::primitives::Blob;
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
pub struct MedicalRequest {
    #[schemars(description = "medical document image keys on S3")]
    pub document_keys: Vec<String>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, PartialEq, Default, aide::OperationIo, JsonSchema,
)]
pub struct MedicalResponse {
    #[schemars(description = "Detected Height")]
    pub height: Option<f64>,
    #[schemars(description = "Detected Weight")]
    pub weight: Option<f64>,
    #[schemars(description = "Detected BMI")]
    pub bmi: Option<f64>,
    #[schemars(description = "Detected Blood Pressure Systolic")]
    pub blood_pressure_systolic: Option<i64>, // 수축기 혈압
    #[schemars(description = "Detected Blood Pressure Diastolic")]
    pub blood_pressure_diastolic: Option<i64>, // 이완기 혈압
}

#[derive(Clone)]
pub struct MedicalHandlerState {
    pub pool: PgPool,
    pub bedrock_client: BedrockClient,
    pub s3_client: S3Client,
}

pub async fn extract_medical_info_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(state): State<MedicalHandlerState>,
    Json(req): Json<MedicalRequest>,
) -> Result<Json<MedicalResponse>> {
    extract_user_id(&state.pool, auth).await?;
    let medical_info = worker(&state, &req).await;
    if let Err(e) = state.s3_client.delete_objects(req.document_keys).await {
        tracing::error!("Failed to delete S3 objects {:?}", e);
    }

    Ok(Json(medical_info?))
}

async fn worker(state: &MedicalHandlerState, req: &MedicalRequest) -> Result<MedicalResponse> {
    let mut contents: Vec<ContentBlock> = vec![];

    for key in req.document_keys.iter() {
        let object = state.s3_client.get_object_bytes(key).await?;
        let format: ImageFormat = match object.content_type {
            Some(S3ContentType::Png) => ImageFormat::Png,
            Some(S3ContentType::Jpeg) => ImageFormat::Jpeg,
            _ => {
                tracing::error!("Unsupported content type for object {}", key);
                return Err(Error::AssetError("Unsupported content type".to_string()));
            }
        };
        let image = ImageBlock::builder()
            .format(format)
            .source(aws_sdk_bedrockruntime::types::ImageSource::Bytes(
                Blob::new(object.data),
            ))
            .build()
            .map_err(|e| {
                tracing::error!("Failed to build ImageBlock for object {}: {:?}", key, e);
                Error::AssetError("Failed to build ImageBlock".to_string())
            })?;

        let content = ContentBlock::Image(image);
        contents.push(content);
    }

    let prompt = r#"
        You are an expert AI specializing in extracting data from medical documents.
        From the provided image of a medical check-up report, please extract the following metrics: `height`, `weight`, `bmi`, `blood_pressure_systolic`, and `blood_pressure_diastolic`.

        **Instructions:**
        1.  **Crucially, analyze the visual layout.** A value is only valid if it is located on the **same row or in the same column** as its corresponding label (e.g., '신장', 'Height'). If a value is not spatially adjacent to its label, consider it unrelated and treat the data as missing by using `null`.
        2.  Analyze the entire image to find the required values. They might be labeled in Korean (e.g., 신장, 체중, 혈압).
        3.  `height` should be in centimeters (cm).
        4.  `weight` should be in kilograms (kg).
        5.  Blood pressure is often written as "systolic/diastolic" (e.g., 120/80). Extract them into `blood_pressure_systolic` and `blood_pressure_diastolic` respectively.
        6.  If a value is not present in the document (or not adjacent to its label), use `null`.
        7.  Your response **must** be a single, raw JSON object. Do not include any explanations, comments, or markdown formatting like ```json.

        **-- Example 1 (Partial Data) --**
        **Input Image Content (Hypothetical):**
        [Image containing the text "신체 계측", "신장(cm): 175.5", "체중(kg): 72.3", "혈압(mmHg): 118/78" but BMI is missing]

        **JSON Output:**
        `{"height":175.5,"weight":72.3,"bmi":null,"blood_pressure_systolic":118,"blood_pressure_diastolic":78}`

        **-- Example 2 (No Data) --**
        **Input Image Content (Hypothetical):**
        [An image of a landscape, containing no medical text or data.]

        **JSON Output:**
        `{"height":null,"weight":null,"bmi":null,"blood_pressure_systolic":null,"blood_pressure_diastolic":null}`

        **-- Task --**
        Now, analyze the provided image and generate the JSON output based on all the instructions above.
        "#;

    let text = state
        .bedrock_client
        .send_message(BedrockModel::NovaLite, prompt.to_string(), Some(contents))
        .await?;
    tracing::debug!("Bedrock response: {}", text);

    let info = if let Some(json) = parse_json::<MedicalResponse>(&text) {
        json
    } else {
        tracing::error!("Failed to extract JSON from response: {}", text);
        return Err(Error::MedicalInfoExtractionFailed(
            "Failed to extract JSON from response".to_string(),
        ));
    };

    Ok(info)
}

// pub fn parse_json<T>(text: &str) -> Option<T>
// where
//     T: serde::de::DeserializeOwned,
// {
//     let json_str = if let (Some(start), Some(end)) = (text.find("```json"), text.rfind("```")) {
//         &text[start + 7..end]
//     } else if let (Some(start), Some(end)) = (text.find('{'), text.rfind('}')) {
//         &text[start..=end]
//     } else {
//         return None;
//     };

//     serde_json::from_str(json_str.trim()).ok()
// }
