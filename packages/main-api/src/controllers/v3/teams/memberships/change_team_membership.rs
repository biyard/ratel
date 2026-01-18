use crate::{
    features::{membership::*, payment::*},
    models::{team::Team, team::TeamOwner, user::User},
    services::portone::{Currency, PortOne},
    types::EntityType,
    utils::time::after_days_from_now_rfc_3339,
    *,
};
use by_axum::aide::NoApi;

pub async fn change_team_membership_handler(
    State(AppState {
        dynamo, portone, ..
    }): State<AppState>,
    NoApi(user): NoApi<User>,
    Extension(team): Extension<Team>,
    Json(req): Json<ChangeTeamMembershipRequest>,
) -> Result<Json<ChangeTeamMembershipResponse>> {
    tracing::debug!("change_team_membership_handler request: {:?}", req);
    let cli = &dynamo.client;

    // Check if user is the team owner - only owners can change team membership
    let team_owner = TeamOwner::get(cli, &team.pk, Some(&EntityType::TeamOwner))
        .await?
        .ok_or(Error::TeamNotFound)?;

    if team_owner.user_pk != user.pk {
        return Err(Error::NoPermission);
    }

    // Get or create team membership
    let (team_membership, current_membership) = get_or_create_team_membership(cli, &team).await?;

    if current_membership.tier == req.membership {
        return Err(Error::MembershipAlreadyActive);
    }

    let mut ret = ChangeTeamMembershipResponse {
        renewal_date: now(),
        receipt: None,
        membership: None,
    };

    if req.membership < current_membership.tier {
        let membership = handle_team_downgrade_membership(
            cli,
            &portone,
            &team,
            &team_membership,
            req.membership.clone(),
        )
        .await?;

        ret.renewal_date = team_membership.expired_at + 1;
        ret.membership = Some(membership.into());
    } else {
        let (team_purchase, membership) = handle_team_upgrade_membership(
            cli,
            &portone,
            &team,
            &team_owner,
            &team_membership,
            current_membership,
            req.membership.clone(),
            req.card_info,
            req.currency,
        )
        .await?;

        ret.receipt = Some(team_purchase.into());
        ret.membership = Some(membership.into());
    };

    Ok(Json(ret))
}

/// Get or create team membership (Free tier if none exists)
async fn get_or_create_team_membership(
    cli: &aws_sdk_dynamodb::Client,
    team: &Team,
) -> Result<(TeamMembership, Membership)> {
    let team_membership =
        TeamMembership::get(cli, team.pk.clone(), Some(EntityType::TeamMembership)).await?;

    match team_membership {
        Some(membership) => {
            let membership_pk: Partition = membership.membership_pk.clone().into();
            let current_membership =
                Membership::get(cli, membership_pk, Some(EntityType::Membership))
                    .await?
                    .ok_or(Error::NoMembershipFound)?;
            Ok((membership, current_membership))
        }
        None => {
            // Create Free membership
            let free_membership_pk = Partition::Membership(MembershipTier::Free.to_string());
            let free_membership = Membership::get(
                cli,
                free_membership_pk.clone(),
                Some(EntityType::Membership),
            )
            .await?
            .ok_or(Error::NoMembershipFound)?;

            let team_membership = TeamMembership::new(
                team.pk.clone().into(),
                free_membership.pk.clone().into(),
                free_membership.duration_days,
                free_membership.credits,
            )?;
            team_membership.create(cli).await?;

            Ok((team_membership, free_membership))
        }
    }
}

/// Handle team membership downgrade by scheduling it for next renewal
async fn handle_team_downgrade_membership(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    team: &Team,
    team_membership: &TeamMembership,
    new_tier: MembershipTier,
) -> Result<Membership> {
    tracing::debug!("Scheduling team membership downgrade to {:?}", new_tier);

    // Get the new membership details
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    // Schedule the downgrade by setting next_membership
    let mut updated_membership = team_membership.clone();
    updated_membership.next_membership = Some(new_membership.pk.clone().into());
    updated_membership.updated_at = now();

    // Cancel any scheduled payments for this team
    let team_payment = TeamPayment::get_by_team(cli, team.pk.clone().into()).await?;
    if let Some(payment) = team_payment {
        payment.cancel_scheduled_payments(cli, portone).await?;
    }

    // Save the scheduled downgrade
    updated_membership.upsert(cli).await?;

    notify!(
        "Scheduled team membership downgrade to {:?} for team {:?}, effective at {}",
        new_tier,
        team.pk,
        team_membership.expired_at
    );

    Ok(new_membership)
}

/// Handle team membership upgrade by immediately activating the new tier
async fn handle_team_upgrade_membership(
    cli: &aws_sdk_dynamodb::Client,
    portone: &PortOne,
    team: &Team,
    team_owner: &TeamOwner,
    team_membership: &TeamMembership,
    current_membership: Membership,
    new_tier: MembershipTier,
    card_info: Option<CardInfo>,
    currency: Currency,
) -> Result<(TeamPurchase, Membership)> {
    tracing::debug!("Processing team membership upgrade to {:?}", new_tier);

    // Get the new membership details
    let new_membership = Membership::get_by_membership_tier(cli, &new_tier).await?;

    // Get or create team payment
    let (team_payment, should_update) =
        TeamPayment::get_or_create(cli, portone, team, team_owner, card_info).await?;

    let tx_type = TransactionType::PurchaseMembership(new_tier.clone());

    let amount = match &currency {
        Currency::Usd => new_membership.price_dollars,
        Currency::Krw => new_membership.price_won,
    };

    // Calculate prorated amount
    let remaining_duration_days = team_membership.calculate_remaining_duration_days();
    let remaining_price =
        current_membership.calculate_remaining_price(currency.clone(), remaining_duration_days);

    let amount = amount - remaining_price;

    // Create a purchase record
    let team_purchase = team_payment
        .purchase(portone, tx_type.clone(), amount, currency.clone())
        .await?;

    let new_team_membership = TeamMembership::new(
        team.pk.clone().into(),
        new_membership.pk.clone().into(),
        new_membership.duration_days,
        new_membership.credits,
    )?;

    let mut txs = vec![
        team_purchase.create_transact_write_item(),
        new_team_membership.upsert_transact_write_item(),
    ];

    #[cfg(test)]
    {
        // Schedule next payment for testing
        let next_time_to_pay = after_days_from_now_rfc_3339(new_membership.duration_days as i64);
        let next_team_purchase = team_payment
            .schedule_next_membership_purchase(portone, tx_type, amount, currency, next_time_to_pay)
            .await?;
        txs.push(next_team_purchase.create_transact_write_item());
    }

    if should_update {
        txs.push(team_payment.upsert_transact_write_item());
    }

    transact_write_all_items_with_failover!(cli, txs);

    Ok((team_purchase, new_membership))
}
