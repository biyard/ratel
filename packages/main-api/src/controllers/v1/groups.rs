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

use crate::security::check_group_perm;
use crate::utils::users::extract_user_id;

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct GroupIdPath {
    pub team_id: i64,
    pub id: i64,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct GroupEmailIdPath {
    pub team_id: i64,
    pub id: i64,
    pub email: String,
}

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct GroupPath {
    pub team_id: i64,
}

#[derive(Clone, Debug)]
pub struct GroupController {
    repo: GroupRepository,
    group_member_repo: GroupMemberRepository,

    pool: sqlx::Pool<sqlx::Postgres>,
}

impl GroupController {
    async fn has_permission(
        &self,
        auth: Option<Authorization>,
        team_id: i64,
    ) -> Result<(i64, Team)> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let team = Team::query_builder()
            .id_equals(team_id)
            .query()
            .map(Team::from)
            .fetch_one(&self.pool)
            .await?;

        tracing::debug!("team data: {:?} {:?}", team.members, user_id);

        let is_member = team.members.iter().any(|member| member.id == user_id);

        if !is_member && team.id != user_id {
            return Err(Error::Unauthorized);
        }
        Ok((user_id, team))
    }

    async fn check_email(
        &self,
        team_id: i64,
        id: i64,
        GroupCheckEmailRequest { email }: GroupCheckEmailRequest,
    ) -> Result<Group> {
        let _team_id = team_id;
        let user = User::query_builder()
            .email_equals(email)
            .query()
            .map(User::from)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to query user: {:?}", e);
                Error::NotFound
            })?
            .ok_or(Error::NotFound)?;

        let exists = GroupMember::query_builder()
            .group_id_equals(id)
            .user_id_equals(user.id)
            .query()
            .map(GroupMember::from)
            .fetch_optional(&self.pool)
            .await?;

        if exists.is_some() {
            return Err(Error::UserAlreadyExists);
        }

        Ok(Group::default())
    }

    async fn invite_member(
        &self,
        team_id: i64,
        id: i64,
        auth: Option<Authorization>,
        GroupInviteMemberRequest { user_ids }: GroupInviteMemberRequest,
    ) -> Result<Group> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let (_user_id, _team) = self.has_permission(auth.clone(), team_id).await?;

        //INFO: checking group permission
        check_group_perm(
            &self.pool,
            auth,
            RatelResource::InviteMember {
                team_id: team_id,
                group_id: id,
            },
            GroupPermission::InviteMember,
        )
        .await?;

        let mut tx = self.pool.begin().await?;

        for user_id in user_ids {
            // let _ = match User::query_builder()
            //     .id_equals(user_id)
            //     .query()
            //     .map(User::from)
            //     .fetch_one(&self.pool)
            //     .await
            // {
            //     Ok(user) => user,
            //     Err(e) => {
            //         tracing::error!("insert user failed with error: {:?}", e);
            //         return Err(Error::InvalidUser);
            //     }
            // };

            let _ = match TeamMember::get_repository(self.pool.clone())
                .insert_with_tx(&mut *tx, team_id, user_id)
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("insert to team member failed with error: {:?}", e);
                }
            };

            let _ = GroupMember::get_repository(self.pool.clone())
                .insert_with_tx(&mut *tx, user_id, id)
                .await?;
        }

        tx.commit().await?;

        Ok(Group::default())
    }

    async fn update(
        &self,
        team_id: i64,
        id: i64,
        auth: Option<Authorization>,
        param: GroupUpdateRequest,
    ) -> Result<Group> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let (_user_id, _team) = self.has_permission(auth.clone(), team_id).await?;
        check_group_perm(
            &self.pool,
            auth,
            RatelResource::UpdateGroup {
                team_id: team_id,
                group_id: id,
            },
            GroupPermission::UpdateGroup,
        )
        .await?;
        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, team_id: i64, id: i64, auth: Option<Authorization>) -> Result<Group> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let (_user_id, _team) = self.has_permission(auth.clone(), team_id).await?;
        check_group_perm(
            &self.pool,
            auth,
            RatelResource::DeleteGroup {
                team_id: team_id,
                group_id: id,
            },
            GroupPermission::DeleteGroup,
        )
        .await?;

        let res = self.repo.delete(id).await?;

        Ok(res)
    }

    async fn query(
        &self,
        _auth: Option<Authorization>,
        param: GroupQuery,
    ) -> Result<QueryResponse<GroupSummary>> {
        let mut total_count = 0;
        let items: Vec<GroupSummary> = Group::query_builder()
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
        team_id: i64,
        GroupCreateRequest {
            name,
            description,
            image_url,
            users,
            permissions,
        }: GroupCreateRequest,
    ) -> Result<Group> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let (_user_id, _team) = self.has_permission(auth.clone(), team_id).await?;
        let mut tx = self.pool.begin().await?;

        let perms: i64 = GroupPermissions(permissions).into();

        let group = self
            .repo
            .insert_with_tx(&mut *tx, name, description, image_url, team_id, perms)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create group: {:?}", e);
                Error::DuplicatedGroupName
            })?
            .ok_or(Error::DuplicatedGroupName)?;

        let group_id = group.id;

        let _ = self
            .group_member_repo
            .insert_with_tx(&mut *tx, team_id, group_id)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create group member: {:?}", e);
                Error::InsertGroupMemberFailed
            })?
            .ok_or(Error::InsertGroupMemberFailed)?;

        for user_id in users {
            let _ = self
                .group_member_repo
                .insert_with_tx(&mut *tx, user_id, group_id)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to create group member: {:?}", e);
                    Error::InsertGroupMemberFailed
                })?
                .ok_or(Error::InsertGroupMemberFailed)?;
        }

        tx.commit().await?;

        Ok(group)
    }
}

impl GroupController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = Group::get_repository(pool.clone());
        let group_member_repo = GroupMember::get_repository(pool.clone());

        Self {
            repo,
            pool,
            group_member_repo,
        }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route(
                "/:id",
                get(Self::get_group_by_id).post(Self::act_group_by_id),
            )
            .with_state(self.clone())
            .route("/", post(Self::act_group).get(Self::get_group))
            .with_state(self.clone()))
    }

    pub async fn get_group_by_id(
        State(ctrl): State<GroupController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Path(GroupIdPath { team_id, id }): Path<GroupIdPath>,
    ) -> Result<Json<Group>> {
        let _team_id = team_id;
        tracing::debug!("get_group {:?}", id);

        Ok(Json(
            Group::query_builder()
                .id_equals(id)
                .query()
                .map(Group::from)
                .fetch_one(&ctrl.pool)
                .await?,
        ))
    }

    pub async fn act_group_by_id(
        State(ctrl): State<GroupController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(GroupIdPath { team_id, id }): Path<GroupIdPath>,
        Json(body): Json<GroupByIdAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act_group_by_id {:?} {:?}", id, body);
        match body {
            GroupByIdAction::Delete(_) => {
                let res = ctrl.delete(team_id, id, auth).await?;
                Ok(Json(res))
            }
            GroupByIdAction::Update(param) => {
                let res = ctrl.update(team_id, id, auth, param).await?;
                Ok(Json(res))
            }
            GroupByIdAction::InviteMember(param) => {
                let res = ctrl.invite_member(team_id, id, auth, param).await?;
                Ok(Json(res))
            }
            //FIXME: fix to read action
            GroupByIdAction::CheckEmail(param) => {
                let res = ctrl.check_email(team_id, id, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_group(
        State(ctrl): State<GroupController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<GroupParam>,
        Path(GroupPath { team_id }): Path<GroupPath>,
    ) -> Result<Json<GroupGetResponse>> {
        let _team_id = team_id;
        tracing::debug!("list groups: {:?}", q);

        match q {
            GroupParam::Query(param) => Ok(Json(GroupGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            // GroupParam::Read(param) => Ok(Json(GroupGetResponse::Read(
            //     ctrl.check_email(auth, param).await?,
            // ))),
        }
    }

    pub async fn act_group(
        State(ctrl): State<GroupController>,
        Extension(auth): Extension<Option<Authorization>>,
        Path(team_id): Path<i64>,
        Json(body): Json<GroupAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act group {:?}", body);
        match body {
            GroupAction::Create(param) => {
                let res = ctrl.create(auth, team_id, param).await?;
                Ok(Json(res))
            }
        }
    }
}
