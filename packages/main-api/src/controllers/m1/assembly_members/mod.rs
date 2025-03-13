use crate::utils::openapi::*;
use bdk::prelude::*;
use by_axum::axum::{extract::State, routing::post};
use dto::*;

#[derive(Clone, Debug)]
pub struct AssemblyMemberControllerM1 {
    repo: AssemblyMemberRepository,
}

impl AssemblyMemberControllerM1 {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = AssemblyMember::get_repository(pool);
        let ctrl = AssemblyMemberControllerM1 { repo };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_assembly_member))
            .with_state(ctrl.clone()))
    }

    pub async fn act_assembly_member(State(ctrl): State<AssemblyMemberControllerM1>) -> Result<()> {
        ctrl.fetch_members().await?;

        Ok(())
    }
}

impl AssemblyMemberControllerM1 {
    async fn fetch_members(&self) -> Result<()> {
        let members = get_active_members().await?;
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
}
