use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    ArtworkCertification, CertificationVoter, Result,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct GetArtworkCertificateRequest {
    #[schemars(description = "ID of the artwork to retrieve")]
    pub artwork_id: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, JsonSchema)]
pub struct CertificationResult {
    pub total_oracles: i64,
    pub total_votes: i64,
    pub approved_votes: i64,
    pub rejected_votes: i64,
    pub certified_at: i64,
    pub voters: Vec<CertificationVoter>,
}

pub async fn get_artwork_certificate_handler(
    Extension(_auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Path(GetArtworkCertificateRequest { artwork_id }): Path<GetArtworkCertificateRequest>,
) -> Result<Json<CertificationResult>> {
    // Get certification data directly from artwork_certifications table
    let certification = ArtworkCertification::query_builder()
        .artwork_id_equals(artwork_id)
        .query()
        .map(ArtworkCertification::from)
        .fetch_one(&pool)
        .await?;

    Ok(Json(CertificationResult {
        total_oracles: certification.total_oracles,
        total_votes: certification.total_votes,
        approved_votes: certification.approved_votes,
        rejected_votes: certification.rejected_votes,
        certified_at: certification.created_at,
        voters: certification.voters,
    }))
}
