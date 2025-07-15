use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use by_types::QueryResponse;
use dto::*;
use sqlx::postgres::PgRow;

use crate::utils::{
    referal_code::generate_referral_code,
    users::{extract_user, extract_user_id},
};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct BotPath {
    pub id: i64,
}

#[derive(Clone, Debug)]
pub struct BotController {
    repo: BotRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl BotController {
    async fn has_permission(&self, auth: Option<Authorization>, bot_id: i64) -> Result<(i64, Bot)> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let bot = Bot::query_builder()
            .id_equals(bot_id)
            .query()
            .map(Bot::from)
            .fetch_one(&self.pool)
            .await?;

        // FIXME: check if the user is a member of the team.
        if bot.parent_id != user_id {
            return Err(Error::Unauthorized);
        }

        Ok((user_id, bot))
    }

    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: BotQuery,
    ) -> Result<QueryResponse<Bot>> {
        let user_id = extract_user_id(&self.pool, _auth).await?;

        let mut total_count = 0;
        let items: Vec<Bot> = Bot::query_builder()
            .parent_id_equals(user_id)
            .limit(param.size())
            .page(param.page())
            .query()
            .map(|row: PgRow| {
                use sqlx::Row;

                total_count = row.try_get("total_count").unwrap_or_default();
                row.into()
            })
            .fetch_all(&self.pool)
            .await?;

        Ok(QueryResponse { total_count, items })
    }

    async fn create(
        &self,
        auth: Option<Authorization>,
        BotCreateRequest {
            profile_url,
            username,
        }: BotCreateRequest,
    ) -> Result<Bot> {
        let user = extract_user(&self.pool, auth).await?;
        let user_id = user.id;
        let repo = User::get_repository(self.pool.clone());
        let bot = repo
            .insert(
                username.clone(),
                username.clone(),
                username.clone(),
                profile_url,
                false,
                false,
                UserType::Bot,
                Some(user_id),
                username.clone(),
                "".to_string(),
                username,
                "".to_string(),
                user.membership,
                generate_referral_code(),
                None,
            )
            .await?;

        Ok(bot.into())
    }

    async fn update_profile_image(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: BotUpdateProfileImageRequest,
    ) -> Result<Bot> {
        self.has_permission(auth, id).await?;

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn update_name(
        &self,
        id: i64,
        auth: Option<Authorization>,
        param: BotUpdateNameRequest,
    ) -> Result<Bot> {
        self.has_permission(auth, id).await?;

        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Bot> {
        self.has_permission(auth, id).await?;

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    // async fn run_read_action(
    //     &self,
    //     _auth: Option<Authorization>,
    //     BotReadAction { action, .. }: BotReadAction,
    // ) -> Result<Bot> {
    //     todo!()
    // }
}

impl BotController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Bot::get_repository(pool.clone());

        Self { repo, pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/:id", get(Self::get_bot_by_id).post(Self::act_bot_by_id))
            .with_state(self.clone())
            .route("/", post(Self::act_bot).get(Self::get_bot))
            .with_state(self.clone()))
    }

    pub async fn act_bot(
        State(ctrl): State<BotController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<BotAction>,
    ) -> Result<Json<Bot>> {
        tracing::debug!("act_bot {:?}", body);
        match body {
            BotAction::Create(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_bot_by_id(
        State(ctrl): State<BotController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(BotPath { id }): Path<BotPath>,
        Json(body): Json<BotByIdAction>,
    ) -> Result<Json<Bot>> {
        tracing::debug!("act_bot_by_id {:?} {:?}", id, body);
        match body {
            BotByIdAction::UpdateProfileImage(param) => {
                let res = ctrl.update_profile_image(id, auth, param).await?;
                Ok(Json(res))
            }
            BotByIdAction::UpdateName(param) => {
                let res = ctrl.update_name(id, auth, param).await?;
                Ok(Json(res))
            }
            BotByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_bot_by_id(
        State(ctrl): State<BotController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(BotPath { id }): Path<BotPath>,
    ) -> Result<Json<Bot>> {
        tracing::debug!("get_bot {:?}", id);

        Ok(Json(
            Bot::query_builder()
                .id_equals(id)
                .query()
                .map(Bot::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn get_bot(
        State(ctrl): State<BotController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<BotParam>,
    ) -> Result<Json<BotGetResponse>> {
        tracing::debug!("list_bot {:?}", q);

        match q {
            BotParam::Query(param) => {
                Ok(Json(BotGetResponse::Query(ctrl.query(auth, param).await?)))
            } // BotParam::Read(param)
              //     if param.action == Some(BotReadActionType::ActionType) =>
              // {
              //     let res = ctrl.run_read_action(auth, param).await?;
              //     Ok(Json(BotGetResponse::Read(res)))
              // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{TestContext, setup};

    #[tokio::test]
    async fn test_create_bot() {
        let TestContext {
            user,
            endpoint,
            now,
            ..
        } = setup().await.unwrap();
        let profile_url = "https://test.com/bot".to_string();
        let username = format!("create-bot-{now}");

        let bot = Bot::get_client(&endpoint)
            .create(profile_url.clone(), username.clone())
            .await
            .unwrap();

        assert_eq!(bot.username, username);
        assert_eq!(bot.profile_url, profile_url);
        assert_eq!(bot.parent_id, user.id);
    }

    #[tokio::test]
    async fn test_update_profile_image() {
        let TestContext {
            user,
            endpoint,
            now,
            ..
        } = setup().await.unwrap();
        let profile_url = "https://test.com/bot".to_string();
        let username = format!("update-profile-bot-{now}");

        let bot = Bot::get_client(&endpoint)
            .create(profile_url.clone(), username.clone())
            .await
            .unwrap();

        assert_eq!(bot.username, username);
        assert_eq!(bot.profile_url, profile_url);
        assert_eq!(bot.parent_id, user.id);

        let new_profile_url = "https://test.com/new_bot".to_string();
        let updated_bot = Bot::get_client(&endpoint)
            .update_profile_image(bot.id, new_profile_url.clone())
            .await
            .unwrap();

        assert_eq!(updated_bot.id, bot.id);
        assert_eq!(updated_bot.username, username);
        assert_eq!(updated_bot.profile_url, new_profile_url);
        assert_eq!(updated_bot.parent_id, user.id);
    }

    #[tokio::test]
    async fn test_update_bot_name() {
        let TestContext {
            user,
            endpoint,
            now,
            ..
        } = setup().await.unwrap();
        let profile_url = "https://test.com/bot".to_string();
        let username = format!("update-name-bot-{now}");

        let bot = Bot::get_client(&endpoint)
            .create(profile_url.clone(), username.clone())
            .await
            .unwrap();

        assert_eq!(bot.username, username);
        assert_eq!(bot.profile_url, profile_url);
        assert_eq!(bot.parent_id, user.id);

        let new_bot_name = format!("update-new-name-bot-{now}");
        let updated_bot = Bot::get_client(&endpoint)
            .update_name(bot.id, new_bot_name.clone())
            .await
            .unwrap();

        assert_eq!(updated_bot.id, bot.id);
        assert_eq!(updated_bot.username, new_bot_name);
        assert_eq!(updated_bot.profile_url, profile_url);
        assert_eq!(updated_bot.parent_id, user.id);
    }

    #[tokio::test]
    async fn test_list_bots() {
        let TestContext {
            user,
            endpoint,
            now,
            ..
        } = setup().await.unwrap();
        let profile_url = "https://test.com/bot".to_string();
        let username = format!("list-bot-{now}");

        let bot = Bot::get_client(&endpoint)
            .create(profile_url.clone(), username.clone())
            .await
            .unwrap();

        assert_eq!(bot.username, username);
        assert_eq!(bot.profile_url, profile_url);
        assert_eq!(bot.parent_id, user.id);

        let bots = Bot::get_client(&endpoint)
            .query(BotQuery::new(10))
            .await
            .unwrap();

        assert_eq!(bots.items.len(), 1);
        let username = format!("list-bot2-{now}");

        Bot::get_client(&endpoint)
            .create(profile_url.clone(), username.clone())
            .await
            .unwrap();

        let bots = Bot::get_client(&endpoint)
            .query(BotQuery::new(10))
            .await
            .unwrap();

        assert_eq!(bots.items.len(), 2);
    }
}
