use crate::models::feed::{Post, PostQueryOption};
use crate::types::{PostStatus, TeamGroupPermission, Visibility};
use crate::utils::dynamo_extractor::extract_user;
use crate::utils::security::{RatelResource, check_any_permission};
use crate::{AppState, Error2};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema, Validate)]
pub struct ListPostsQueryParams {
    #[validate(range(min = 1, max = 100))]
    pub limit: Option<i32>,
    pub bookmark: Option<String>,

    pub r#type: ListPostType,
    pub value: Option<String>, // PK of User or Team based on type
    pub status: Option<PostStatus>,
}

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub enum ListPostType {
    All,
    Me,
    User,
    Team,
}

// SORT Created At
// Post Status: Draft, Published
// Visibility: Private, Public, Team(team_pk)

// anonymous users : Scope: Any, Status: Published, Visibility: Public
// my draft posts : Scope: user_pk == logined_user_pk, Status: Draft, Visibility: Any
// my published posts : Scope: user_pk == logined_user_pk, Status: Published, Visibility: Any
// another user's posts : Scope: user_pk == target_user_pk, Status : Published, Visibility: Public
// team's posts (as a non-member): Scope: user_pk == Team PK, Status: Published, Visibility: Public
// team's draft posts (as a member): Scope: user_pk == Team PK, Status: Draft, Visibility: Any
// team's published posts (as a member): Scope: user_pk == Team PK, Status: Published, Visibility: Any

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub next_bookmark: Option<String>,
}

pub async fn list_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Query(params): Query<ListPostsQueryParams>,
) -> Result<Json<ListPostsResponse>, Error2> {
    if let Err(e) = params.validate() {
        return Err(Error2::BadRequest(e.to_string()));
    }

    let mut query_options = PostQueryOption::builder();
    query_options = if let Some(limit) = params.limit {
        query_options.limit(limit)
    } else {
        query_options.limit(20)
    };

    if let Some(bookmark) = params.bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, next_bookmark) = match params.r#type {
        ListPostType::All => {
            Post::find_by_visibility(&dynamo.client, Visibility::Public, query_options).await?
        }
        ListPostType::Me => {
            let user = extract_user(&dynamo.client, auth).await?;
            let user_pk = user.pk;
            let status = params.status.unwrap_or(PostStatus::Published);
            query_options = query_options.sk(status.to_string());

            Post::find_by_user_pk(&dynamo.client, &user_pk, query_options).await?
        }
        ListPostType::User => {
            let user_pk = params.value.ok_or(Error2::BadRequest(
                "value (user_pk) is required for type User".to_string(),
            ))?;
            query_options = query_options.sk(Visibility::Public.to_string());

            Post::find_by_user_pk_visibility(&dynamo.client, user_pk, query_options).await?
        }
        ListPostType::Team => {
            let team_pk = params.value.ok_or(Error2::BadRequest(
                "value (team_pk) is required for type Team".to_string(),
            ))?;

            // If team try to access draft, or User have permission to read the team,
            let mut has_permission = false;
            if check_any_permission(
                &dynamo.client,
                auth,
                RatelResource::Team {
                    team_pk: team_pk.clone(),
                },
                vec![
                    TeamGroupPermission::PostRead,
                    TeamGroupPermission::PostWrite,
                    TeamGroupPermission::PostEdit,
                ],
            )
            .await
            .is_ok()
            {
                has_permission = true;
            }

            match params.status {
                Some(PostStatus::Draft) => {
                    if !has_permission {
                        return Err(Error2::Unauthorized(
                            "You do not have permission to access draft posts".to_string(),
                        ));
                    }
                    query_options = query_options.sk(PostStatus::Draft.to_string());

                    Post::find_by_user_pk(&dynamo.client, &team_pk, query_options).await?
                }
                _ => {
                    if has_permission {
                        query_options = query_options.sk(PostStatus::Published.to_string());
                        Post::find_by_user_pk(&dynamo.client, &team_pk, query_options).await?
                    } else {
                        query_options = query_options.sk(Visibility::Public.to_string());

                        Post::find_by_user_pk_visibility(&dynamo.client, &team_pk, query_options)
                            .await?
                    }
                }
            }
        }
    };

    Ok(Json(ListPostsResponse {
        posts,
        next_bookmark,
    }))
}
