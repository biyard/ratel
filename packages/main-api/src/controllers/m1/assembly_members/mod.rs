use crate::utils::openapi::*;
use bdk::prelude::*;
use by_axum::axum::{extract::State, routing::post};
use dto::*;

const BATCH_SIZE: u32 = 100;
const MAX_PROPOSER_SUM: u32 = 9000; // 2025.03.19: 8270

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerM1 {
    pool: sqlx::Pool<sqlx::Postgres>,
    repo: AssemblyMemberRepository,
    prop: ProposerRepository,
}

impl AssemblyMemberControllerM1 {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        let repo = AssemblyMember::get_repository(pool.clone());
        let prop = Proposer::get_repository(pool.clone());
        Self { pool, repo, prop }
    }

    pub fn route(&self) -> by_axum::axum::Router {
        by_axum::axum::Router::new()
            .route("/", post(Self::act_assembly_member))
            .with_state(self.clone())
            .route("/proposers", post(Self::act_proposers))
            .with_state(self.clone())
    }

    pub async fn act_assembly_member(State(ctrl): State<AssemblyMemberControllerM1>) -> Result<()> {
        ctrl.fetch_members().await?;

        Ok(())
    }

    pub async fn act_proposers(State(ctrl): State<AssemblyMemberControllerM1>) -> Result<()> {
        ctrl.fetch_proposers().await?;

        Ok(())
    }
}

impl AssemblyMemberControllerM1 {
    async fn fetch_members(&self) -> Result<()> {
        let members = fetch_active_members().await?;
        tracing::debug!("members: {:?}", members);

        for member in members {
            let image_url = get_member_profile_image(member.code.clone()).await?;
            tracing::debug!("image_url: {:?}", image_url);
            let en_member = get_active_member_en(member.code.clone()).await?;
            tracing::debug!("en_member: {:?}", en_member);

            match self
                .repo
                .insert(
                    member.code,
                    member.name,
                    member.party,
                    member.district,
                    en_member.name,
                    en_member.party,
                    en_member.district,
                    CryptoStance::default(),
                    image_url,
                    member.email,
                )
                .await
            {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("error: {:?}", e);
                }
            }
        }

        Ok(())
    }

    async fn fetch_proposers(&self) -> Result<()> {
        for i in 1..=MAX_PROPOSER_SUM / BATCH_SIZE {
            let proposers = fetch_proposers(i, BATCH_SIZE).await?;

            for proposer in proposers {
                tracing::debug!("proposer: {:?}", proposer);

                let bill: Bill = match Bill::query_builder()
                    .bill_no_equals(proposer.bill_no.clone())
                    .query()
                    .map(|r: sqlx::postgres::PgRow| r.into())
                    .fetch_one(&self.pool)
                    .await
                {
                    Ok(bill) => bill,
                    Err(e) => {
                        tracing::error!("error: {:?}", e);
                        continue;
                    }
                };

                let rst_proposers = proposer
                    .representative_name
                    .split(",")
                    .collect::<Vec<&str>>();

                for name in rst_proposers {
                    // their are no same name in proposers in 22nd assembly members (if it's what I know)

                    let rep_member: AssemblyMember = AssemblyMember::query_builder()
                        .name_equals(name.to_string())
                        .query()
                        .map(|r: sqlx::postgres::PgRow| r.into())
                        .fetch_one(&self.pool)
                        .await?;

                    match self.prop.insert(rep_member.id, bill.id, true).await {
                        Ok(_) => {}
                        Err(e) => {
                            tracing::error!("error: {:?}", e);
                            continue;
                        }
                    }
                }

                let pub_proposers = proposer.proposer_names.split(",").collect::<Vec<&str>>();

                for name in pub_proposers {
                    let pub_member: AssemblyMember = AssemblyMember::query_builder()
                        .name_equals(name.to_string())
                        .query()
                        .map(|r: sqlx::postgres::PgRow| r.into())
                        .fetch_one(&self.pool)
                        .await?;

                    match self.prop.insert(pub_member.id, bill.id, false).await {
                        Ok(_) => {}
                        Err(e) => {
                            tracing::error!("error: {:?}", e);
                            continue;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
