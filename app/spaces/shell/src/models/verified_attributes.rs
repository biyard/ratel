use crate::models::{Age, Gender, RespondentAttr};
use crate::*;

pub use common::models::did::VerifiedAttributes;

#[cfg(feature = "server")]
pub async fn get_attributes(
    dynamo: &common::utils::aws::dynamo::DynamoClient,
    user_pk: Partition,
) -> Result<Option<RespondentAttr>> {
    let res = VerifiedAttributes::get(
        &dynamo,
        CompositePartition(user_pk, Partition::Attributes),
        None::<String>,
    )
    .await
    .unwrap_or_default()
    .unwrap_or(VerifiedAttributes::default());

    let age = if res.age().is_none() {
        None
    } else {
        match res.age().unwrap_or_default() {
            0..=17 => Some(Age::Range {
                inclusive_max: 17,
                inclusive_min: 0,
            }),
            18..=29 => Some(Age::Range {
                inclusive_max: 29,
                inclusive_min: 18,
            }),
            30..=39 => Some(Age::Range {
                inclusive_max: 39,
                inclusive_min: 30,
            }),
            40..=49 => Some(Age::Range {
                inclusive_max: 49,
                inclusive_min: 40,
            }),
            50..=59 => Some(Age::Range {
                inclusive_max: 59,
                inclusive_min: 50,
            }),
            60..=69 => Some(Age::Range {
                inclusive_max: 69,
                inclusive_min: 60,
            }),
            _ => Some(Age::Range {
                inclusive_max: 100,
                inclusive_min: 70,
            }),
        }
    };

    let gender = res.gender.map(|value| match value {
        common::attribute::Gender::Male => Gender::Male,
        common::attribute::Gender::Female => Gender::Female,
    });
    let school = res.university;

    Ok(Some(RespondentAttr {
        age,
        gender,
        school,
    }))
}

#[cfg(feature = "server")]
#[async_trait::async_trait]
pub trait UserAttributesExt {
    async fn get_attributes(&self, cli: &aws_sdk_dynamodb::Client) -> Result<VerifiedAttributes>;
}

#[cfg(feature = "server")]
#[async_trait::async_trait]
impl UserAttributesExt for ratel_auth::User {
    async fn get_attributes(&self, cli: &aws_sdk_dynamodb::Client) -> Result<VerifiedAttributes> {
        let (pk, sk) = VerifiedAttributes::keys(&self.pk);
        Ok(VerifiedAttributes::get(cli, pk, Some(sk))
            .await?
            .unwrap_or_default())
    }
}
