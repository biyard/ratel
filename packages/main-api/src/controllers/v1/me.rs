use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
        routing::get,
    },
};
use dto::*;

use crate::utils::users::extract_user_id;

#[derive(Clone, Debug)]
pub struct MeController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl MeController {
    async fn my_info(&self, auth: Option<Authorization>) -> Result<MyInfo> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let res = MyInfo::query_builder()
            .id_equals(user_id)
            .query()
            .map(MyInfo::from)
            .fetch_one(&self.pool)
            .await?;

        Ok(res)
    }
}

impl MeController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/info", get(Self::get_my_info))
            .with_state(self.clone()))
    }

    pub async fn get_my_info(
        State(ctrl): State<MeController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<MyInfoParam>,
    ) -> Result<Json<MyInfoGetResponse>> {
        tracing::debug!("list_me {:?}", q);

        match q {
            MyInfoParam::Read(param) if param.action == Some(MyInfoReadActionType::MyInfo) => {
                let res = ctrl.my_info(auth).await?;
                Ok(Json(MyInfoGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}

#[cfg(test)]
mod tests {
    use bdk::prelude::sqlx::{Pool, Postgres};

    use super::*;
    use crate::tests::{TestContext, setup};

    async fn test_setup(user: &User, admin: &User, pool: Pool<Postgres>, now: i64) -> (User, User) {
        let repo = User::get_repository(pool.clone());
        let username = format!("test_user_{}", now);
        let profile_url = format!("https://example.com/{}", username);
        let user_id = user.id;
        let admin_id = admin.id;

        let mut tx = pool.begin().await.unwrap();

        let team = repo
            .insert_with_tx(
                &mut *tx,
                "".to_string(),
                username.clone(),
                username.clone(),
                profile_url.clone(),
                false,
                false,
                UserType::Team,
                Some(user_id),
                username,
                "".to_string(),
                format!("0x{:40x}", now + 2), // unique evm_address
                "".to_string(), // password
            )
            .await
            .unwrap()
            .unwrap();

        TeamMember::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, team.id, user_id)
            .await
            .unwrap();
        let username = format!("test_admin_{}", now);

        let team2 = repo
            .insert_with_tx(
                &mut *tx,
                "".to_string(),
                username.clone(),
                username.clone(),
                profile_url.clone(),
                false,
                false,
                UserType::Team,
                Some(admin_id),
                username,
                "".to_string(),
                format!("0x{:40x}2", now + 2), // unique evm_address
                "".to_string(), // password
            )
            .await
            .unwrap()
            .unwrap();

        TeamMember::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, team2.id, admin_id)
            .await
            .unwrap();

        TeamMember::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, team2.id, user_id)
            .await
            .unwrap();

        let username = format!("test_admin2_{}", now);

        let team3 = repo
            .insert_with_tx(
                &mut *tx,
                "".to_string(),
                username.clone(),
                username.clone(),
                profile_url,
                false,
                false,
                UserType::Team,
                Some(admin_id),
                username,
                "".to_string(),
                format!("0x{:40x}3", now + 3), // unique evm_address
                "".to_string(), // password
            )
            .await
            .unwrap()
            .unwrap();

        TeamMember::get_repository(pool.clone())
            .insert_with_tx(&mut *tx, team3.id, admin_id)
            .await
            .unwrap();

        tx.commit().await.unwrap();

        (team, team2)
    }

    #[tokio::test]
    async fn test_get_my_teams() {
        let TestContext {
            user,
            admin,
            now,
            endpoint,
            pool,
            ..
        } = setup().await.unwrap();
        let (team, team2) = test_setup(&user, &admin, pool, now).await;

        let my_info = MyInfo::get_client(&endpoint).my_info().await.unwrap();

        assert_eq!(my_info.id, user.id);
        assert_eq!(my_info.nickname, user.nickname);
        assert_eq!(my_info.principal, user.principal);
        assert_eq!(my_info.email, user.email);
        assert_eq!(my_info.profile_url, user.profile_url);
        assert_eq!(my_info.teams.len(), 2);
        assert_eq!(my_info.teams[0].id, team.id);
        assert_eq!(my_info.teams[0].created_at, team.created_at);
        assert_eq!(my_info.teams[0].updated_at, team.updated_at);
        assert_eq!(my_info.teams[0].parent_id, user.id);

        assert_eq!(my_info.teams[1].id, team2.id);
        assert_eq!(my_info.teams[1].created_at, team2.created_at);
        assert_eq!(my_info.teams[1].updated_at, team2.updated_at);
        assert_eq!(my_info.teams[1].parent_id, admin.id);
    }
}
