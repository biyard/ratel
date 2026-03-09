use crate::features::posts::controllers::dto::*;
use crate::features::posts::models::*;
use crate::features::posts::types::*;
use crate::features::posts::*;
use ratel_auth::User;

#[get("/api/posts/drafts?bookmark", user: User)]
pub async fn list_user_drafts_handler(
    bookmark: Option<String>,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    tracing::debug!("list_user_drafts_handler: bookmark = {:?}", bookmark);

    let mut query_options = Post::opt().limit(10).sk(PostStatus::Draft.to_string());

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) = Post::find_by_user_and_status(cli, &user.pk, query_options).await?;

    tracing::debug!(
        "list_user_drafts_handler: found {} drafts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse::from((Some(user.clone()), post)))
        .collect();

    Ok(ListItemsResponse { items, bookmark })
}

#[get("/api/teams/:teamname/drafts?bookmark", user: User)]
pub async fn list_team_drafts_handler(
    teamname: String,
    bookmark: Option<String>,
) -> Result<ListItemsResponse<PostResponse>> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();
    tracing::debug!("list_user_drafts_handler: bookmark = {:?}", bookmark);

    let opt = Team::opt().limit(1);
    let (teams, _): (Vec<Team>, _) = Team::find_by_username_prefix(cli, &teamname, opt).await?;
    let team = teams
        .into_iter()
        .next()
        .ok_or(Error::NotFound(format!("Team not found: {}", teamname)))?;
    let team_pk = team.pk;

    let mut query_options = Post::opt().limit(10).sk(PostStatus::Draft.to_string());

    if let Some(bookmark) = bookmark.clone() {
        query_options = query_options.bookmark(bookmark);
    }

    let (posts, bookmark) = Post::find_by_user_and_status(cli, &team_pk, query_options).await?;

    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| PostResponse::from((Some(user.clone()), post)))
        .collect();

    Ok(ListItemsResponse { items, bookmark })
}
