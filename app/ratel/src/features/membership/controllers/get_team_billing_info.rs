use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::get_billing_info::BillingInfoResponse;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::models::TeamPayment;
use crate::features::membership::*;
use crate::features::posts::models::Team;
use crate::features::social::pages::member::dto::TeamRole;

#[get("/v3/teams/:username/billing", user: User, team: Team, role: TeamRole)]
pub async fn get_team_billing_info_handler(username: String) -> Result<BillingInfoResponse> {
    let result = async {
        if !role.is_admin_or_owner() {
            return Err(Error::NotFound("Permission denied".to_string()));
        }

        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();

        let pk = CompositePartition::team_payment_pk(team.pk.into());
        let payment: Option<TeamPayment> = TeamPayment::get(cli, &pk, None::<String>).await?;

        match payment {
            Some(p) => Ok(BillingInfoResponse {
                has_card: p.billing_key.is_some(),
                customer_name: p.name,
                masked_card_number: p.masked_card_number,
            }),
            None => Ok(BillingInfoResponse::default()),
        }
    }
    .await;

    result.map_err(normalize_error)
}
