use super::super::*;

#[get("/api/users/:username/pk")]
pub async fn resolve_user_pk_handler(username: String) -> Result<String> {
    let cli = super::super::config::get().dynamodb();

    let (users, _) =
        crate::features::auth::User::find_by_username(cli, &username, Default::default()).await?;

    let user = users
        .into_iter()
        .next()
        .ok_or(crate::common::Error::NotFound(format!(
            "User not found: {}",
            username
        )))?;

    Ok(user.pk.to_string())
}
