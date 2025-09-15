use crate::{security::check_perm, utils::users::extract_user_id};
use bdk::prelude::*;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::post},
};
use dto::{by_axum::axum::extract::Path, *};

#[derive(Clone, Debug)]
pub struct SprintLeagueController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SprintLeaguePath {
    pub space_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SprintLeagueByIdPath {
    pub space_id: i64,
    pub sprint_league_id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SprintLeaguePlayerPath {
    pub space_id: i64,
    pub sprint_league_id: i64,
    pub player_id: i64,
}

impl SprintLeagueController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", post(Self::act))
            .route("/:sprint-league-id", post(Self::act_by_id))
            .route(
                "/:sprint-league-id/players/:player-id",
                post(Self::act_player),
            )
            .with_state(self.clone())
    }

    pub async fn act(
        State(ctrl): State<SprintLeagueController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SprintLeaguePath { space_id }): Path<SprintLeaguePath>,
        Json(body): Json<SprintLeagueAction>,
    ) -> Result<Json<SprintLeague>> {
        match body {
            SprintLeagueAction::Create(param) => {
                let sprint_league = ctrl.create(space_id, auth, param).await?;
                Ok(Json(sprint_league))
            } // Other actions can be added here in the future
              // SprintLeagueAction::Update(param) => {
              //     let sprint_league = ctrl.update(space_id, auth, param).await?;
              //     Ok(Json(sprint_league))
              // }
        }
    }

    pub async fn act_by_id(
        State(ctrl): State<SprintLeagueController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SprintLeagueByIdPath {
            space_id,
            sprint_league_id,
        }): Path<SprintLeagueByIdPath>,
        Json(param): Json<SprintLeagueByIdAction>,
    ) -> Result<Json<SprintLeague>> {
        match param {
            SprintLeagueByIdAction::Vote(SprintLeagueVoteRequest {
                player_id,
                referral_code,
            }) => {
                let res = ctrl
                    .vote(auth, space_id, sprint_league_id, player_id, referral_code)
                    .await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn act_player(
        State(ctrl): State<SprintLeagueController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(SprintLeaguePlayerPath {
            space_id,
            sprint_league_id,
            player_id,
        }): Path<SprintLeaguePlayerPath>,
        Json(param): Json<SprintLeaguePlayerUpdateRequest>,
    ) -> Result<Json<SprintLeaguePlayer>> {
        let player = ctrl
            .update_player(space_id, sprint_league_id, player_id, auth, param)
            .await?;
        Ok(Json(player))
    }
}

impl SprintLeagueController {
    async fn create(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
        param: SprintLeagueCreateRequest,
    ) -> Result<SprintLeague> {
        check_perm(
            &self.pool,
            auth.clone(),
            RatelResource::Space { space_id },
            GroupPermission::ManageSpace,
        )
        .await?;

        let repo = SprintLeague::get_repository(self.pool.clone());
        let player_repo = SprintLeaguePlayer::get_repository(self.pool.clone());

        let mut tx = self.pool.begin().await?;
        let sprint_league = repo
            .insert_with_tx(&mut *tx, space_id, param.reward_amount)
            .await?;
        if sprint_league.is_none() {
            return Err(Error::SprintLeagueCreationFailed);
        }
        let sprint_league = sprint_league.unwrap();

        for player in param.players {
            player_repo
                .insert_with_tx(
                    &mut *tx,
                    sprint_league.id,
                    player.name,
                    player.description,
                    player.player_images,
                )
                .await?;
        }
        tx.commit().await?;
        Ok(sprint_league)
    }

    // async fn update(
    //     &self,
    //     space_id: i64,
    //     auth: Option<Authorization>,
    //     param: SprintLeagueUpdateRequest,
    // ) -> Result<SprintLeague> {
    //     check_perm(
    //         &self.pool,
    //         auth.clone(),
    //         RatelResource::Space { space_id },
    //         GroupPermission::ManageSpace,
    //     )
    //     .await?;
    //     let space = Space::query_builder()
    //         .id_equals(space_id)
    //         .query()
    //         .map(Space::from)
    //         .fetch_one(&self.pool)
    //         .await?;
    //     if space.status != SpaceStatus::Draft {
    //         return Err(Error::BadRequest);
    //     }

    //     let repo = SprintLeague::get_repository(self.pool.clone());
    //     let mut tx = self.pool.begin().await?;
    //     let sprint_league = repo
    //         .update_with_tx(
    //             &mut *tx,
    //             space_id,
    //             SprintLeagueRepositoryUpdateRequest {
    //                 ..Default::default()
    //             },
    //         )
    //         .await?;
    //     if sprint_league.is_none() {
    //         return Err(Error::SprintLeagueUpdateFailed);
    //     }
    //     tx.commit().await?;
    //     Ok(sprint_league.unwrap())
    // }

    async fn vote(
        &self,
        auth: Option<Authorization>,
        space_id: i64,
        sprint_league_id: i64,
        sprint_league_player_id: i64,
        referral_code: Option<String>,
    ) -> Result<SprintLeague> {
        let user_id = extract_user_id(&self.pool, auth).await.unwrap_or_default();
        tracing::debug!(
            "Voting in sprint league: space_id={}, sprint_league_id={}, player_id={}, user_id={}",
            space_id,
            sprint_league_id,
            sprint_league_player_id,
            user_id
        );
        let space = Space::query_builder(user_id)
            .sprint_leagues_builder(
                SprintLeague::query_builder(user_id)
                    .players_builder(SprintLeaguePlayer::query_builder()),
            )
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool)
            .await?;

        let sprint_league = space.sprint_leagues.first().ok_or(Error::NotFound)?;
        let now = chrono::Utc::now().timestamp();
        if space.status != SpaceStatus::InProgress
            || sprint_league.is_voted
            || (space.ended_at.is_some() && now >= space.ended_at.unwrap_or_default())
            || sprint_league.id != sprint_league_id
        {
            return Err(Error::BadRequest);
        }

        let repo = SprintLeagueVote::get_repository(self.pool.clone());
        repo.insert(
            referral_code,
            user_id,
            sprint_league_id,
            sprint_league_player_id,
        )
        .await?;

        Ok(sprint_league.clone())
    }

    async fn update_player(
        &self,
        space_id: i64,
        sprint_league_id: i64,
        player_id: i64,
        auth: Option<Authorization>,
        param: SprintLeaguePlayerUpdateRequest,
    ) -> Result<SprintLeaguePlayer> {
        tracing::debug!(
            "Updating player: space_id={}, sprint_league_id={}, player_id={} param={:?}",
            space_id,
            sprint_league_id,
            player_id,
            param
        );
        check_perm(
            &self.pool,
            auth.clone(),
            RatelResource::Space { space_id },
            GroupPermission::ManageSpace,
        )
        .await?;

        // Note:
        // Is this the best way to check if the space is in draft status?
        // It might be better to change permission level after vote start...

        let space = Space::query_builder(0)
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool)
            .await?;

        if space.status != SpaceStatus::Draft {
            return Err(Error::BadRequest);
        }

        SprintLeaguePlayer::query_builder()
            .sprint_league_id_equals(sprint_league_id)
            .id_equals(player_id)
            .query()
            .map(SprintLeaguePlayer::from)
            .fetch_one(&self.pool)
            .await?;

        let repo = SprintLeaguePlayer::get_repository(self.pool.clone());
        let mut tx = self.pool.begin().await?;
        let player = repo
            .update_with_tx(&mut *tx, player_id, {
                SprintLeaguePlayerRepositoryUpdateRequest {
                    name: Some(param.name),
                    description: Some(param.description),
                    player_images: Some(param.player_images),
                    ..Default::default()
                }
            })
            .await?;

        tx.commit().await?;
        Ok(player.unwrap())
    }
}
