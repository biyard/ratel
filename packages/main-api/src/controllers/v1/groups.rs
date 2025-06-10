#![allow(unused)]
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

use crate::{security::check_perm, utils::users::extract_user_id};

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
pub struct GroupPath {
    pub id: i64,
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
        group_id: i64,
    ) -> Result<(i64, Group)> {
        let user_id = extract_user_id(&self.pool, auth).await?;

        let group = Group::query_builder()
            .id_equals(group_id)
            .query()
            .map(Group::from)
            .fetch_one(&self.pool)
            .await?;

        let is_member = group.members.iter().any(|member| member.id == user_id);

        if !is_member {
            return Err(Error::Unauthorized);
        }
        Ok((user_id, group))
    }

    async fn invite_member(
        &self,
        id: i64,
        auth: Option<Authorization>,
        GroupInviteMemberRequest { user_ids }: GroupInviteMemberRequest,
    ) -> Result<Group> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let (_user_id, _group) = self.has_permission(auth, id).await?;
        let mut tx = self.pool.begin().await?;

        for user_id in user_ids {
            let user = match User::query_builder()
                .id_equals(user_id)
                .query()
                .map(User::from)
                .fetch_one(&mut *tx)
                .await
            {
                Ok(user) => user,
                Err(_) => {
                    return Err(Error::InvalidUser);
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
        id: i64,
        auth: Option<Authorization>,
        param: GroupUpdateRequest,
    ) -> Result<Group> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let (_user_id, _group) = self.has_permission(auth, id).await?;
        let res = self.repo.update(id, param.into()).await?;

        Ok(res)
    }

    async fn delete(&self, id: i64, auth: Option<Authorization>) -> Result<Group> {
        if auth.is_none() {
            return Err(Error::Unauthorized);
        }
        let (_user_id, _group) = self.has_permission(auth, id).await?;

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
        GroupCreateRequest {
            name,
            description,
            image_url,
            users,
            permissions,
        }: GroupCreateRequest,
    ) -> Result<Group> {
        let user_id = extract_user_id(&self.pool, auth).await?;
        let mut tx = self.pool.begin().await?;

        let perms: i64 = GroupPermissions(permissions).into();

        let group = self
            .repo
            .insert_with_tx(&mut *tx, name, description, image_url, user_id, perms)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create group: {:?}", e);
                Error::DuplicatedGroupName
            })?
            .ok_or(Error::DuplicatedGroupName)?;

        let group_id = group.id;
        for user in users {
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
        Path(GroupPath { id }): Path<GroupPath>,
    ) -> Result<Json<Group>> {
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
        Path(GroupPath { id }): Path<GroupPath>,
        Json(body): Json<GroupByIdAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act_group_by_id {:?} {:?}", id, body);
        match body {
            GroupByIdAction::Delete(_) => {
                let res = ctrl.delete(id, auth).await?;
                Ok(Json(res))
            }
            GroupByIdAction::Update(param) => {
                let res = ctrl.update(id, auth, param).await?;
                Ok(Json(res))
            }
            GroupByIdAction::InviteMember(param) => {
                let res = ctrl.invite_member(id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    pub async fn get_group(
        State(ctrl): State<GroupController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<GroupParam>,
    ) -> Result<Json<GroupGetResponse>> {
        tracing::debug!("list groups: {:?}", q);

        match q {
            GroupParam::Query(param) => Ok(Json(GroupGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
            _ => Err(Error::BadRequest),
        }
    }

    pub async fn act_group(
        State(ctrl): State<GroupController>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<GroupAction>,
    ) -> Result<Json<Group>> {
        tracing::debug!("act group {:?}", body);
        match body {
            GroupAction::Create(param) => {
                let res = ctrl.create(auth, param).await?;
                Ok(Json(res))
            }
        }
    }
}
