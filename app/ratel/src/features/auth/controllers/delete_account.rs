use crate::features::auth::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[cfg(feature = "server")]
const DEFAULT_PROFILE_URL: &str = "https://metadata.ratel.foundation/ratel/default-profile.png";
#[cfg(feature = "server")]
const ANON_AUTHOR_NAME: &str = "Deleted User";
#[cfg(feature = "server")]
const ANON_AUTHOR_USERNAME: &str = "deleted";
/// Safety cap on author-snapshot anonymization pages so a user with an
/// unbounded post/comment history can't make the delete request run forever.
#[cfg(feature = "server")]
const MAX_ANON_PAGES: usize = 40;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct DeleteAccountResponse {
    pub status: String,
}

/// Soft-delete the current user's account. Irreversible from the user's side.
///
/// - Stamps `deleted_at` and anonymizes `email`/`username`/`display_name`/
///   `profile_url` so the original email & username are freed for re-signup and
///   the tombstoned row can never be found by the original identifiers.
/// - Removes auth-linkage rows (EVM / OAuth / phone / principal / telegram) so
///   the account can't be resurrected via any login method.
/// - Anonymizes the author snapshot on the user's posts and comments (content
///   stays in context; the deleted identity is hidden).
/// - Flushes the current session.
#[post("/api/auth/account/delete", user: User, session: Extension<tower_sessions::Session>)]
pub async fn delete_account_handler() -> Result<DeleteAccountResponse> {
    let Extension(session) = session;
    let conf = crate::features::auth::config::get();
    let cli = conf.dynamodb();

    let uid = user.id();
    let now = crate::common::utils::time::now();

    // 1) Soft-delete + anonymize the user row. Updating `email`/`username`
    //    rewrites their GSI keys, so the original values stop resolving and
    //    become available for a fresh signup.
    User::updater(user.pk.clone(), user.sk.clone())
        .with_deleted_at(now)
        .with_email(format!("deleted+{uid}@deleted.ratel"))
        .with_username(format!("deleted_{uid}"))
        .with_display_name(ANON_AUTHOR_NAME.to_string())
        .with_profile_url(DEFAULT_PROFILE_URL.to_string())
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("delete_account: anonymize user {uid} failed: {e}");
            AuthError::AccountDeletionFailed
        })?;

    // 2) Remove auth-linkage rows so re-auth via any method can't resurrect the
    //    account. Best-effort: a missing row is fine, and a failure here must
    //    not leave the user half-deleted (the row above is already tombstoned).
    let _ = UserEvmAddress::delete(cli, user.pk.clone(), Some(EntityType::UserEvmAddress)).await;
    let _ = UserOAuth::delete(cli, user.pk.clone(), Some(EntityType::UserOAuth)).await;
    let _ =
        UserPhoneNumber::delete(cli, user.pk.clone(), Some(EntityType::UserPhoneNumber)).await;
    let _ = UserPrincipal::delete(cli, user.pk.clone(), Some(EntityType::UserPrincipal)).await;
    let _ = UserTelegram::delete(cli, user.pk.clone(), Some(EntityType::UserTelegram)).await;

    // 3) Anonymize author snapshots on the user's posts & comments (content
    //    preserved, identity hidden). Best-effort — never fails the deletion.
    anonymize_user_posts(cli, &user.pk).await;
    anonymize_user_comments(cli, &user.pk).await;

    // 4) Flush the current session.
    let _ = session.flush().await;

    Ok(DeleteAccountResponse {
        status: "OK".to_string(),
    })
}

#[cfg(feature = "server")]
async fn anonymize_user_posts(cli: &aws_sdk_dynamodb::Client, user_pk: &Partition) {
    use crate::features::posts::models::Post;

    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_ANON_PAGES {
        let opts = Post::opt_with_bookmark(bookmark.clone()).limit(50);
        let (items, next) = match Post::find_by_user_pk(cli, user_pk, opts).await {
            Ok(v) => v,
            Err(e) => {
                crate::error!("delete_account: list posts failed: {e}");
                return;
            }
        };

        for p in items {
            if let Err(e) = Post::updater(p.pk.clone(), p.sk.clone())
                .with_author_display_name(ANON_AUTHOR_NAME.to_string())
                .with_author_username(ANON_AUTHOR_USERNAME.to_string())
                .with_author_profile_url(DEFAULT_PROFILE_URL.to_string())
                .execute(cli)
                .await
            {
                crate::error!("delete_account: anonymize post {:?} failed: {e}", p.pk);
            }
        }

        match next {
            Some(b) => bookmark = Some(b),
            None => return,
        }
    }
}

#[cfg(feature = "server")]
async fn anonymize_user_comments(cli: &aws_sdk_dynamodb::Client, user_pk: &Partition) {
    use crate::features::posts::models::PostComment;

    let mut bookmark: Option<String> = None;
    for _ in 0..MAX_ANON_PAGES {
        let opts = PostComment::opt_with_bookmark(bookmark.clone()).limit(50);
        let (items, next) = match PostComment::find_by_user_pk(cli, user_pk, opts).await {
            Ok(v) => v,
            Err(e) => {
                crate::error!("delete_account: list comments failed: {e}");
                return;
            }
        };

        for c in items {
            if let Err(e) = PostComment::updater(c.pk.clone(), c.sk.clone())
                .with_author_display_name(ANON_AUTHOR_NAME.to_string())
                .with_author_username(ANON_AUTHOR_USERNAME.to_string())
                .with_author_profile_url(DEFAULT_PROFILE_URL.to_string())
                .execute(cli)
                .await
            {
                crate::error!("delete_account: anonymize comment {:?} failed: {e}", c.pk);
            }
        }

        match next {
            Some(b) => bookmark = Some(b),
            None => return,
        }
    }
}
