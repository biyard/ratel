use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpaceRecommendation {
    pub pk: Partition,
    pub sk: EntityType,

    pub html_contents: String,
    pub files: Vec<File>,
}

impl SpaceRecommendation {
    pub fn new(pk: Partition, html_contents: String, files: Vec<File>) -> Self {
        let sk = EntityType::SpaceRecommendation;

        Self {
            pk,
            sk,
            html_contents,
            files,
        }
    }

    pub fn keys(space_pk: &Partition) -> (Partition, EntityType) {
        (space_pk.clone(), EntityType::SpaceRecommendation)
    }

    pub async fn update_files(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        files: Vec<File>,
    ) -> Result<Self, crate::Error> {
        let (pk, sk) = Self::keys(&space_pk);

        let recommendation = SpaceRecommendation::get(&cli, pk.clone(), Some(sk.clone())).await?;

        if recommendation.is_none() {
            SpaceRecommendation::new(pk.clone(), "".to_string(), files.clone())
                .create(&cli)
                .await?;
        } else {
            SpaceRecommendation::updater(pk.clone(), sk.clone())
                .with_files(files.clone())
                .execute(&cli)
                .await?;
        }

        let mut recommendation = recommendation.unwrap_or_default();
        recommendation.files = files;

        Ok(recommendation)
    }

    pub async fn update_contents(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        html_contents: String,
    ) -> Result<Self, crate::Error> {
        let (pk, sk) = Self::keys(&space_pk);

        let recommendation = SpaceRecommendation::get(&cli, pk.clone(), Some(sk.clone())).await?;

        if recommendation.is_none() {
            SpaceRecommendation::new(pk.clone(), html_contents.clone(), vec![])
                .create(&cli)
                .await?;
        } else {
            SpaceRecommendation::updater(pk.clone(), sk.clone())
                .with_html_contents(html_contents.clone())
                .execute(&cli)
                .await?;
        }

        let mut recommendation = recommendation.unwrap_or_default();
        recommendation.html_contents = html_contents;

        Ok(recommendation)
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct SpaceRecommendationResponse {
    pub html_contents: String,
    pub files: Vec<File>,
}

impl From<SpaceRecommendation> for SpaceRecommendationResponse {
    fn from(recommendation: SpaceRecommendation) -> Self {
        Self {
            html_contents: recommendation.clone().html_contents,
            files: recommendation.clone().files,
        }
    }
}
