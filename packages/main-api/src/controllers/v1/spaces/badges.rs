use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStream;
use by_axum::{
    aide,
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Path, Query, State},
        routing::{get, post},
    },
};
use dto::{by_types::QueryResponse, sqlx::postgres::PgRow, *};
use ethers::providers::{Http, Provider};

use crate::{
    config,
    security::check_perm,
    utils::{
        contracts::erc1155::Erc1155Contract,
        users::{extract_user, extract_user_with_allowing_anonymous},
        wallets::{kaia_local_wallet::KaiaLocalWallet, local_fee_payer::LocalFeePayer},
    },
};

#[derive(Clone, Debug)]
pub struct SpaceBadgeController {
    repo: SpaceBadgeRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
    provider: Arc<Provider<Http>>,

    owner: KaiaLocalWallet,
    feepayer: LocalFeePayer,
    cli: aws_sdk_s3::Client,
}

impl SpaceBadgeController {
    // async fn query(
    //     &self,
    //     space_id: i64,
    //     _auth: Option<Authorization>,
    //     param: SpaceBadgeQuery,
    // ) -> Result<QueryResponse<Badge>> {
    //     let mut total_count = 0;
    //     let items: Vec<SpaceBadgeSummary> = SpaceBadgeSummary::query_builder()
    //         .limit(param.size())
    //         .page(param.page())
    //         .space_id_equals(space_id)
    //         .query()
    //         .map(|row: PgRow| {
    //             use sqlx::Row;

    //             total_count = row.try_get("total_count").unwrap_or_default();
    //             row.into()
    //         })
    //         .fetch_all(&self.pool)
    //         .await?;

    //     Ok(QueryResponse { total_count, items })
    // }

    async fn query(
        &self,
        auth: Option<Authorization>,
        param: SpaceBadgeQuery,
    ) -> Result<QueryResponse<UserBadge>> {
        let mut total_count = 0;

        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;

        tracing::debug!("111 size: {} page: {}", param.size(), param.page());

        let items = UserBadge::query_builder()
            .user_id_equals(user.id)
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
        space_id: i64,
        auth: Option<Authorization>,
        SpaceBadgeCreateRequest { badges }: SpaceBadgeCreateRequest,
    ) -> Result<SpaceBadge> {
        let repo = Badge::get_repository(self.pool.clone());
        let user = extract_user(&self.pool, auth.clone())
            .await
            .unwrap_or_default();
        let user = check_perm(
            &self.pool,
            auth,
            RatelResource::Space {
                team_id: user.id,
                space_id,
            },
            GroupPermission::ManageSpace,
        )
        .await?;
        let creator_id = user.id;
        let mut tx = self.pool.begin().await?;

        for b in badges.clone() {
            let BadgeCreateRequest {
                name,
                image_url,
                contract,
                token_id,
            } = b;

            let badge = repo
                .insert_with_tx(
                    &mut *tx,
                    creator_id,
                    name,
                    Scope::Space,
                    image_url,
                    contract,
                    token_id,
                )
                .await?
                .ok_or(Error::BadgeCreationFailure)?;

            self.repo
                .insert_with_tx(&mut *tx, space_id, badge.id)
                .await?
                .map(SpaceBadge::from)
                .ok_or(Error::BadgeCreationFailure)?;
        }

        tx.commit().await?;

        let c = &config::get().bucket;
        let contract_address = badges[0].contract.clone().unwrap_or_default();

        let mut ids = vec![];
        let mut values = vec![];

        for b in badges.iter() {
            let path = format!(
                "{}/json/{}/{}/{}.json",
                c.asset_dir,
                config::get().env,
                space_id,
                b.token_id.unwrap_or_default()
            );

            tracing::debug!("Uploading to s3: {}", path);
            match self
                .cli
                .put_object()
                .bucket(c.name)
                .key(&path)
                .body(ByteStream::from(
                    serde_json::json!({
                        "name": format!("{} #{}", b.name, b.token_id.unwrap_or_default()),
                        "image": b.image_url,
                        "animation_url": b.image_url,

                    })
                    .to_string()
                    .as_bytes()
                    .to_vec(),
                ))
                .content_type("application/json")
                .send()
                .await
            {
                Ok(_) => {
                    ids.push(b.token_id.unwrap_or_default() as u64);
                    values.push(1);
                    tracing::debug!("Uploaded to s3: {}", path);
                }
                Err(e) => {
                    tracing::warn!(
                        "Failed to upload to s3 for {}: {}",
                        b.token_id.unwrap_or_default(),
                        e
                    );
                }
            }
        }

        let mut contract = Erc1155Contract::new(&contract_address, self.provider.clone());
        contract.set_wallet(self.owner.clone());
        contract.set_fee_payer(self.feepayer.clone());

        contract.mint_batch(contract_address, ids, values).await?;

        Ok(SpaceBadge::default())
    }

    async fn claim(
        &self,
        space_id: i64,
        auth: Option<Authorization>,
        SpaceBadgeClaimRequest { ids, evm_address }: SpaceBadgeClaimRequest,
    ) -> Result<SpaceBadge> {
        let user = extract_user_with_allowing_anonymous(&self.pool, auth).await?;
        tracing::debug!("Claiming badges for user: {:?}", user);

        let badges =
            sqlx::query("SELECT * FROM user_badges WHERE user_id = $1 AND badge_id = ANY($2)")
                .bind(user.id)
                .bind(&ids)
                .map(UserBadge::from)
                .fetch_all(&self.pool)
                .await?;

        tracing::debug!("Claimed badges: {:?}", badges);

        if badges.len() == ids.len() {
            return Err(Error::AlreadyClaimed);
        }

        let ids = ids
            .into_iter()
            .filter(|id| !badges.iter().any(|b| b.badge_id == *id))
            .collect::<Vec<_>>();

        if ids.is_empty() {
            tracing::debug!("No new badges to claim for user: {}", user.id);
            return Err(Error::AlreadyClaimed);
        }

        tracing::debug!("Filtered badge IDs: {:?}", ids);

        let space = BadgesOfSpace::query_builder()
            .id_equals(space_id)
            .query()
            .map(BadgesOfSpace::from)
            .fetch_one(&self.pool)
            .await?;

        tracing::debug!("Space badges: {:?}", space.badges);

        let mut token_ids = vec![];
        let mut values = vec![];
        let mut badge_ids = vec![];

        let contract_address = space
            .badges
            .first()
            .and_then(|b| b.contract.clone())
            .ok_or(Error::BadgeCreationFailure)?;

        for b in space.badges.iter() {
            if ids.contains(&b.id) {
                badge_ids.push(b.id);
                token_ids.push(b.token_id.unwrap_or_default() as u64);
                values.push(1);
                if b.contract.is_none()
                    || contract_address != b.contract.clone().unwrap_or_default()
                {
                    return Err(Error::BadgeCreationFailure);
                }
            }
        }

        if token_ids.is_empty() {
            tracing::debug!("No valid badges to mint for user: {}", user.id);
            return Err(Error::BadgeCreationFailure);
        }

        let mut contract = Erc1155Contract::new(&contract_address, self.provider.clone());
        contract.set_wallet(self.owner.clone());
        contract.set_fee_payer(self.feepayer.clone());

        tracing::debug!(
            "Minting badges({:?}) for user: {} with contract: {}",
            token_ids,
            user.id,
            contract_address
        );

        contract.mint_batch(evm_address, token_ids, values).await?;

        let mut tx = self.pool.begin().await?;

        let repo = UserBadge::get_repository(self.pool.clone());

        for badge_id in badge_ids {
            repo.insert_with_tx(&mut *tx, user.id, badge_id).await?;
        }
        tx.commit().await?;

        Ok(SpaceBadge::default())
    }
}

impl SpaceBadgeController {
    pub async fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = SpaceBadge::get_repository(pool.clone());

        let conf = config::get();

        use aws_config::BehaviorVersion;
        use aws_config::{Region, defaults};
        use aws_sdk_s3::config::Credentials;

        let config = defaults(BehaviorVersion::latest())
            .region(Region::new(conf.aws.region))
            .credentials_provider(Credentials::new(
                conf.aws.access_key_id,
                conf.aws.secret_access_key,
                None,
                None,
                "credential",
            ));

        let config = config.load().await;
        let cli = aws_sdk_s3::Client::new(&config);
        let provider = Provider::<Http>::try_from(conf.kaia.endpoint).unwrap();
        let provider = Arc::new(provider);

        let owner = KaiaLocalWallet::new(conf.kaia.owner_key, provider.clone())
            .await
            .expect("Failed to create owner wallet");
        let feepayer = LocalFeePayer::new(
            conf.kaia.feepayer_address,
            conf.kaia.feepayer_key,
            provider.clone(),
        )
        .await
        .expect("Failed to create fee payer wallet");

        Self {
            repo,
            pool,
            cli,
            owner,
            feepayer,
            provider,
        }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            // .route(
            //     "/:id",
            //     // get(Self::get_space_badge_by_id).
            //     post(Self::act_space_badge_by_id),
            // )
            // .with_state(self.clone())
            .route("/", get(Self::get_user_badge))
            .route("/", post(Self::act_space_badge))
            .with_state(self.clone())
    }

    async fn get_user_badge(
        State(ctrl): State<SpaceBadgeController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<SpaceBadgeParam>,
    ) -> Result<Json<UserBadgeGetResponse>> {
        match q {
            SpaceBadgeParam::Query(param) => Ok(Json(UserBadgeGetResponse::Query(
                ctrl.query(auth, param).await?,
            ))),
        }
    }

    pub async fn act_space_badge(
        State(ctrl): State<SpaceBadgeController>,
        Path(SpaceBadgeParentPath { space_id }): Path<SpaceBadgeParentPath>,
        Extension(auth): Extension<Option<Authorization>>,
        Json(body): Json<SpaceBadgeAction>,
    ) -> Result<Json<SpaceBadge>> {
        tracing::debug!("act_space_badge {} {:?}", space_id, body);
        match body {
            SpaceBadgeAction::Create(param) => {
                let res = ctrl.create(space_id, auth, param).await?;
                Ok(Json(res))
            }
            SpaceBadgeAction::Claim(param) => {
                let res = ctrl.claim(space_id, auth, param).await?;
                Ok(Json(res))
            }
        }
    }

    // pub async fn act_space_badge_by_id(
    //     State(ctrl): State<SpaceBadgeController>,
    //     Extension(auth): Extension<Option<Authorization>>,
    //     Path(SpaceBadgePath { space_id, id }): Path<SpaceBadgePath>,
    //     Json(body): Json<SpaceBadgeByIdAction>,
    // ) -> Result<Json<SpaceBadge>> {
    //     tracing::debug!("act_space_badge_by_id {} {:?} {:?}", space_id, id, body);

    //     match body {}
    // }

    // pub async fn get_space_badge_by_id(
    //     State(ctrl): State<SpaceBadgeController>,
    //     Extension(_auth): Extension<Option<Authorization>>,
    //     Path(SpaceBadgePath { space_id, id }): Path<SpaceBadgePath>,
    // ) -> Result<Json<SpaceBadge>> {
    //     tracing::debug!("get_space_badge {} {:?}", space_id, id);
    //     Ok(Json(
    //         SpaceBadge::query_builder()
    //             .id_equals(id)
    //             .space_id_equals(space_id)
    //             .query()
    //             .map(SpaceBadge::from)
    //             .fetch_one(&ctrl.pool)
    //             .await?,
    //     ))
    // }
}

// #[derive(
//     Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
// )]
// #[serde(rename_all = "kebab-case")]
// pub struct SpaceBadgePath {
//     pub space_id: i64,
//     pub id: i64,
// }

#[derive(
    Debug, Clone, serde::Deserialize, serde::Serialize, schemars::JsonSchema, aide::OperationIo,
)]
#[serde(rename_all = "kebab-case")]
pub struct SpaceBadgeParentPath {
    pub space_id: i64,
}
