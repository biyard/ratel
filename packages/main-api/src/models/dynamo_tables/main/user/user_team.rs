use std::collections::HashMap;

use crate::Error;
use crate::models::email_template::email_template::EmailTemplate;
// #[cfg(all(not(test), not(feature = "no-secret")))]
// use crate::features::spaces::templates::SpaceTemplate;
use crate::User;
use crate::email_operation::EmailOperation;
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::models::SpaceCommon;
use crate::models::UserNotification;
use crate::services::fcm_notification::FCMService;
use crate::utils::aws::DynamoClient;

use crate::{
    models::team::Team,
    types::*,
    utils::{aws::SesClient, time::get_now_timestamp_millis},
};
use bdk::prelude::*;
use serde_json::json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserTeam {
    pub pk: Partition,
    #[dynamo(prefix = "TEAM_PK", index = "gsi1", name = "find_by_team", pk)]
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    pub last_used_at: i64,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,

    pub dao_address: Option<String>,
}

impl UserTeam {
    pub fn new(
        pk: Partition,
        Team {
            display_name,
            profile_url,
            username,
            dao_address,
            pk: team_pk,
            ..
        }: Team,
    ) -> Self {
        let last_used_at = get_now_timestamp_millis();

        Self {
            pk,
            sk: EntityType::UserTeam(team_pk.to_string()),
            last_used_at,
            display_name,
            profile_url,
            username,
            dao_address,
        }
    }

    // #[cfg(all(not(test), not(feature = "no-secret")))]
    // async fn ensure_team_invite_template_exists(
    //     dynamo: &DynamoClient,
    //     ses: &SesClient,
    //     template_name: &str,
    // ) -> Result<(), Error> {
    //     use crate::utils::templates::{INVITE_TEAM_TEMPLATE_HTML, INVITE_TEAM_TEMPLATE_SUBJECT};

    //     let template = SpaceTemplate::get(
    //         &dynamo.client,
    //         Partition::SpaceTemplate,
    //         Some(EntityType::SpaceTemplate(template_name.to_string())),
    //     )
    //     .await?;

    //     if template.is_none() {
    //         ses.create_template(
    //             template_name,
    //             INVITE_TEAM_TEMPLATE_SUBJECT,
    //             INVITE_TEAM_TEMPLATE_HTML,
    //         )
    //         .await
    //         .map_err(|e| Error::AwsSesSendEmailException(e.to_string()))?;

    //         let temp = SpaceTemplate::new(template_name.to_string());
    //         temp.create(&dynamo.client).await?;
    //     }

    //     Ok(())
    // }

    #[allow(unused_variables)]
    pub async fn send_email(
        dynamo: &DynamoClient,
        ses: &SesClient,
        team: Team,
        user_emails: Vec<String>,
    ) -> Result<(), Error> {
        let mut domain = crate::config::get().domain.to_string();
        if domain.contains("localhost") {
            domain = format!("http://{}", domain);
        } else {
            domain = format!("https://{}", domain);
        }

        let url = format!("{}/teams/{}/home", domain, team.username);

        let email = EmailTemplate {
            targets: user_emails.clone(),
            operation: EmailOperation::TeamInvite {
                team_name: team.username.clone(),
                team_profile: team.profile_url.clone(),
                team_display_name: team.display_name.clone(),
                url,
            },
        };

        email.send_email(&dynamo, &ses).await?;

        Ok(())
    }

    pub async fn send_notification(
        dynamo: &DynamoClient,
        fcm: &mut FCMService,
        recipients: Vec<Partition>,
        team: &Team,
    ) -> Result<(), Error> {
        if recipients.is_empty() {
            tracing::info!(
                "UserTeam::send_notification: no recipients, skip push (team_pk={})",
                team.pk
            );
            return Ok(());
        }

        tracing::info!(
            "UserTeam::send_notification: start, team_pk={}, recipients={}",
            team.pk,
            recipients.len()
        );

        let title = "You are invited to join a team.".to_string();
        let body = format!(
            "Join team {} (@{}) and collaborate together.",
            team.display_name, team.username
        );

        UserNotification::send_to_users(dynamo, fcm, &recipients, title, body, None).await?;

        tracing::info!("UserTeam::send_notification: done for team_pk={}", team.pk);

        Ok(())
    }
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct UserTeamResponse {
    pub nickname: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
    pub dao_address: Option<String>,
}

impl From<UserTeam> for UserTeamResponse {
    fn from(user_team: UserTeam) -> Self {
        Self {
            nickname: user_team.display_name,
            profile_url: user_team.profile_url,
            username: user_team.username,
            dao_address: user_team.dao_address,
            user_type: UserType::Team,
        }
    }
}
