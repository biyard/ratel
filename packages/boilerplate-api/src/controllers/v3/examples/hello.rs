use crate::models::ExampleData;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct HelloRequest {
    #[schemars(description = "Data")]
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct HelloResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn hello_handler(Json(req): Json<HelloRequest>) -> Result<Json<HelloResponse>> {
    let cli = config::ddb();
    tracing::debug!("hello request: {:?}", req);

    ExampleData::new(req.data.clone()).create(&cli).await?;

    Ok(Json(HelloResponse {
        status: format!("Hello, {}!", req.data),
    }))
}
