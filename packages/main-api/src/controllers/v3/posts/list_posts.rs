use crate::models::feed::{Post, PostQueryOption};
use crate::models::user::User;
use crate::types::list_items_response::ListItemsResponse;
use crate::types::{PostStatus, Visibility};
use crate::{AppState, Error2};
use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Query, State},
};
use dto::aide::NoApi;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Validate)]
pub struct ListPostsQueryParams {
    #[validate(range(min = 1, max = 100))]
    #[schemars(description = "Number of items to return (default: 20, max: 100)")]
    pub limit: Option<i32>,
    pub bookmark: Option<String>,

    pub value: Option<String>, // PK of User or Team based on type
    pub status: Option<PostStatus>,
}

pub async fn list_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Query(params): Query<ListPostsQueryParams>,
) -> Result<Json<ListItemsResponse<Post>>, Error2> {
    tracing::debug!("list_posts_handler: user = {:?}", _user);
    if let Err(e) = params.validate() {
        return Err(Error2::BadRequest(e.to_string()));
    }

    let mut query_options = PostQueryOption::builder().limit(params.limit.unwrap_or(20));

    if let Some(bookmark) = params.bookmark {
        query_options = query_options.bookmark(bookmark);
    }
    let posts = Post::find_by_visibility(&dynamo.client, Visibility::Public, query_options).await?;

    if posts.0.is_empty() {
        return Ok(Json(ListItemsResponse {
            items: vec![],
            bookmark: None,
        }));
    }

    // TODO: getting like for a user

    Ok(Json(posts.into()))
}

// let (posts, next_bookmark) = match params.r#type {
//     ListPostType::All => {
//         Post::find_by_visibility(&dynamo.client, Visibility::Public, query_options).await?
//     }
//     ListPostType::Me => {
//         let user = extract_user(&dynamo.client, session).await?;
//         let user_pk = user.pk;
//         let status = params.status.unwrap_or(PostStatus::Published);
//         query_options = query_options.sk(status.to_string());

//         Post::find_by_user_pk(&dynamo.client, &user_pk, query_options).await?
//     }
//     ListPostType::User => {
//         let user_pk = params.value.ok_or(Error2::BadRequest(
//             "value (user_pk) is required for type User".to_string(),
//         ))?;
//         query_options = query_options.sk(Visibility::Public.to_string());

//         Post::find_by_user_pk_visibility(&dynamo.client, user_pk, query_options).await?
//     }
//     ListPostType::Team => {
//         let team_pk = params.value.ok_or(Error2::BadRequest(
//             "value (team_pk) is required for type Team".to_string(),
//         ))?;

//         // If team try to access draft, or User have permission to read the team,
//         let mut has_permission = false;
//         if check_any_permission(
//             &dynamo.client,
//             session,
//             RatelResource::Team {
//                 team_pk: team_pk.clone(),
//             },
//             vec![
//                 TeamGroupPermission::PostRead,
//                 TeamGroupPermission::PostWrite,
//                 TeamGroupPermission::PostEdit,
//             ],
//         )
//         .await
//         .is_ok()
//         {
//             has_permission = true;
//         }

//         match params.status {
//             Some(PostStatus::Draft) => {
//                 if !has_permission {
//                     return Err(Error2::Unauthorized(
//                         "You do not have permission to access draft posts".to_string(),
//                     ));
//                 }
//                 query_options = query_options.sk(PostStatus::Draft.to_string());

//                 Post::find_by_user_pk(&dynamo.client, &team_pk, query_options).await?
//             }
//             _ => {
//                 if has_permission {
//                     query_options = query_options.sk(PostStatus::Published.to_string());
//                     Post::find_by_user_pk(&dynamo.client, &team_pk, query_options).await?
//                 } else {
//                     query_options = query_options.sk(Visibility::Public.to_string());

//                     Post::find_by_user_pk_visibility(&dynamo.client, &team_pk, query_options)
//                         .await?
//                 }
//             }
//         }
//     }
