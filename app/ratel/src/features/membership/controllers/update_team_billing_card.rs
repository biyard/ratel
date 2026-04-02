use super::*;
use crate::features::auth::User;
use crate::features::membership::controllers::get_billing_info::BillingInfoResponse;
use crate::features::membership::controllers::normalize_error;
use crate::features::membership::controllers::update_billing_card::UpdateBillingCardRequest;
use crate::features::membership::models::TeamPayment;
#[cfg(feature = "server")]
use crate::features::membership::services::portone::PortOne;
use crate::features::membership::*;
use crate::features::posts::models::Team;
use crate::features::posts::types::{TeamGroupPermission, TeamGroupPermissions};

#[cfg(feature = "server")]
fn mask_card_number(card_number: &str) -> String {
    let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 4 {
        return "****".to_string();
    }
    let last4 = &digits[digits.len() - 4..];
    format!("****-****-****-{last4}")
}

#[post("/v3/teams/:username/billing", user: User, team: Team, permissions: TeamGroupPermissions)]
pub async fn update_team_billing_card_handler(
    username: String,
    req: UpdateBillingCardRequest,
) -> Result<BillingInfoResponse> {
    let result = async {
        if !permissions.contains(TeamGroupPermission::TeamAdmin) {
            return Err(Error::NotFound("Permission denied".to_string()));
        }

        let conf = crate::features::membership::config::get();
        let cli = conf.common.dynamodb();
        let portone = PortOne::new(conf.portone.api_secret);

        let pk = CompositePartition::team_payment_pk(team.pk.clone().into());
        let payment: TeamPayment = TeamPayment::get(cli, &pk, None::<String>)
            .await?
            .ok_or_else(|| {
                Error::NotFound(
                    "No payment profile found. Please verify your identity first.".to_string(),
                )
            })?;

        let card = req.card_info;
        let masked = mask_card_number(&card.card_number);

        let res = portone
            .get_billing_key(
                payment.customer_id.clone(),
                payment.name.clone(),
                card.card_number,
                card.expiry_year,
                card.expiry_month,
                card.birth_or_business_registration_number,
                card.password_two_digits,
            )
            .await?;

        let new_billing_key = res.billing_key_info.billing_key;
        TeamPayment::updater(&payment.pk, &payment.sk)
            .with_billing_key(new_billing_key)
            .with_masked_card_number(masked.clone())
            .execute(cli)
            .await?;

        Ok(BillingInfoResponse {
            has_card: true,
            customer_name: payment.name,
            masked_card_number: Some(masked),
        })
    }
    .await;

    result.map_err(normalize_error)
}
