use crate::models::feed::Post;
use crate::types::{EntityType, Partition, PostStatus, PostType, TeamGroupPermission, Visibility};
use crate::utils::dynamo_extractor::extract_user;
use crate::utils::security::{RatelResource, check_permission};
use crate::utils::validator::{validate_content, validate_title};
use crate::{AppState, Error2};
use dto::by_axum::{
    auth::Authorization,
    axum::{
        Extension,
        extract::{Json, Path, State},
    },
};
use dto::{JsonSchema, aide, schemars};
use serde::Deserialize;

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub struct UpdatePostPathParams {
    pub post_pk: String,
}

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub enum UpdatePostRequest {
    Post {
        title: String,
        content: String,
        // images: Vec<String>,
    },
    Artwork {
        title: String,
        content: String,
        // images: Vec<String>,
        // metadata: Vec<String>,
    },
    Visibility {
        status: PostStatus,
        visibility: UpdateVisibility,
    },
}

#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
pub enum UpdateVisibility {
    Private,
    Public,
}
// #[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
// pub struct UpdatePostResponse {}
pub type UpdatePostResponse = Post;

pub async fn update_post_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(auth): Extension<Option<Authorization>>,
    Path(params): Path<UpdatePostPathParams>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<UpdatePostResponse>, Error2> {
    if auth.is_none() {
        return Err(Error2::Unauthorized("Authentication required".into()));
    }
    let mut post = Post::get(&dynamo.client, &params.post_pk, Some(EntityType::Post))
        .await?
        .ok_or(Error2::NotFound("Post not found".to_string()))?;
    match post.user_pk {
        Partition::Team(_) => {
            check_permission(
                &dynamo.client,
                auth.clone(),
                RatelResource::Team {
                    team_pk: post.user_pk.to_string(),
                },
                vec![TeamGroupPermission::PostEdit],
            )
            .await?;
        }
        Partition::User(_) => {
            let user = extract_user(&dynamo.client, auth).await?;
            if user.pk != post.user_pk {
                return Err(Error2::Unauthorized(
                    "You do not have permission to update this post".into(),
                ));
            }
        }
        _ => return Err(Error2::InternalServerError("Invalid post author".into())),
    }

    match req {
        UpdatePostRequest::Post { title, content } => {
            validate_title(&title)?;
            validate_content(&content)?;

            Post::updater(&post.pk, &post.sk)
                .with_title(title.clone())
                .with_html_contents(content.clone())
                .with_post_type(PostType::Post)
                .execute(&dynamo.client)
                .await?;
            post.post_type = PostType::Post;
            post.title = title;
            post.html_contents = content;
        }
        UpdatePostRequest::Artwork { title, content } => {
            validate_title(&title)?;
            validate_content(&content)?;

            Post::updater(&post.pk, &post.sk)
                .with_title(title.clone())
                .with_html_contents(content.clone())
                .with_post_type(PostType::Artwork)
                .execute(&dynamo.client)
                .await?;
            post.post_type = PostType::Artwork;
            post.title = title;
            post.html_contents = content;
        }
        UpdatePostRequest::Visibility { status, visibility } => {
            let now = chrono::Utc::now().timestamp_micros();
            if post.status != PostStatus::Draft && status != PostStatus::Published {
                return Err(Error2::BadRequest(
                    "Only Draft posts can be updated to Published".to_string(),
                ));
            }
            let visibility = match visibility {
                UpdateVisibility::Private => Visibility::Team(post.user_pk.to_string()),
                UpdateVisibility::Public => Visibility::Public,
            };
            Post::updater(&post.pk, &post.sk)
                .with_visibility(visibility.clone())
                .with_status(status.clone())
                .with_compose_sort_key(Post::get_compose_key(
                    PostStatus::Published,
                    Some(visibility.clone()),
                    chrono::Utc::now().timestamp_micros(),
                ))
                .with_updated_at(now)
                .execute(&dynamo.client)
                .await?;

            post.updated_at = now;
            post.status = status;
            post.visibility = Some(visibility);
        }
    }

    Ok(Json(post))
}
