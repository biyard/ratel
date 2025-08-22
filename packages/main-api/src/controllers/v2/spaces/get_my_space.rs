#![allow(unused)]
use std::thread::Thread;

use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Feed, GroupPermission, Result, Space,
    by_axum::{auth::Authorization, axum::extract::Path},
    sqlx::{Pool, Postgres},
};

use crate::{
    security::check_perm,
    utils::users::{extract_user, extract_user_id},
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
pub struct SpaceSummary {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub title: Option<String>,
    pub html_contents: String,
    pub image_url: String,
}

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
pub struct GetSpaceResponse {
    pub spaces: Vec<SpaceSummary>,
    pub boostings: Vec<SpaceSummary>,
}

pub async fn get_my_space_controller(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
) -> Result<Json<GetSpaceResponse>> {
    let user_id = extract_user_id(&pool, auth).await.unwrap_or(0);

    let spaces = Space::query_builder(user_id)
        .owner_id_equals(user_id)
        .query()
        .map(Space::from)
        .fetch_all(&pool)
        .await?;

    let mut my_spaces = vec![];
    let mut boostings = vec![
        SpaceSummary {
            id: 800,
            created_at: 1755686527,
            updated_at: 1755686527,
            title: Some("DAO Governance: Community-Driven Decision Making".to_string()),
            html_contents: "<p>Learn how decentralized autonomous organizations (DAOs) enable transparent and democratic governance, giving every token holder a voice in shaping the future.</p>".to_string(),
            image_url: "https://metadata.ratel.foundation/dao.jpg".to_string(),
        },
        SpaceSummary {
            id: 801,
            created_at: 1755686527,
            updated_at: 1755686527,
            title: Some("Web3 Education: Tokenized Learning for All".to_string()),
    html_contents: "<p>Discover how blockchain and tokenomics are transforming education by enabling credential verification, skill-based rewards, and global access to knowledge.</p>".to_string(),
            image_url:"https://metadata.ratel.foundation/web3.jpeg".to_string(),
        },
        SpaceSummary {
            id: 802,
            created_at: 1755686527,
            updated_at: 1755686527,
            title: Some("Sustainable Finance with Carbon Credits".to_string()),
    html_contents: "<p>Explore how carbon-credit-backed stablecoins can support green initiatives while offering a transparent, decentralized way to invest in sustainability.</p>".to_string(),
            image_url: "https://metadata.ratel.foundation/carbon.jpeg".to_string(),
        },
    ];

    for space in spaces {
        let feed_id = space.feed_id;

        let feed = Feed::query_builder(user_id)
            .id_equals(feed_id)
            .query()
            .map(Feed::from)
            .fetch_one(&pool)
            .await?;

        my_spaces.push(SpaceSummary {
            id: space.id,
            created_at: space.created_at,
            updated_at: space.updated_at,
            title: space.title,
            html_contents: space.html_contents,
            image_url: feed.url.unwrap_or_default(),
        });
    }

    Ok(Json(GetSpaceResponse {
        spaces: my_spaces,
        boostings,
    }))
}
