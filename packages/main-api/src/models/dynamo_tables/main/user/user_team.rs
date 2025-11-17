use crate::Error;
// #[cfg(all(not(test), not(feature = "no-secret")))]
// use crate::features::spaces::templates::SpaceTemplate;
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
}

impl UserTeam {
    pub fn new(
        pk: Partition,
        Team {
            display_name,
            profile_url,
            username,
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
        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            for email in &user_emails {
                tracing::warn!("sending email will be skipped for {}", email);
            }
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
            if user_emails.is_empty() {
                return Ok(());
            }

            let template_name = "team_invite".to_string();
            // Self::ensure_team_invite_template_exists(dynamo, ses, &template_name).await?;

            let mut domain = crate::config::get().domain.to_string();
            if domain.contains("localhost") {
                domain = format!("http://{}", domain);
            } else {
                domain = format!("https://{}", domain);
            }

            let url = format!("{}/teams/{}/home", domain, team.username);
            tracing::debug!("team url: {:?}", url);

            let template_data = json!({
                "team_name": team.username,
                "team_profile": team.profile_url,
                "team_display_name": team.display_name,
                "url": url,
            });

            let recipients: Vec<(String, Option<serde_json::Value>)> = user_emails
                .into_iter()
                .map(|email| (email, Some(template_data.clone())))
                .collect();

            let mut i = 0;
            while let Err(e) = ses
                .send_bulk_with_template(&template_name, &recipients)
                .await
            {
                btracing::notify!(
                    crate::config::get().slack_channel_monitor,
                    &format!("Failed to send team invite email: {:?}", e)
                );
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                i += 1;
                if i >= 3 {
                    return Err(Error::AwsSesSendEmailException(e.to_string()));
                }
            }
        }

        Ok(())
    }
}

#[derive(Default, serde::Serialize, schemars::JsonSchema)]
pub struct UserTeamResponse {
    pub nickname: String,
    pub profile_url: String,
    pub username: String,
    pub user_type: UserType,
}

impl From<UserTeam> for UserTeamResponse {
    fn from(user_team: UserTeam) -> Self {
        Self {
            nickname: user_team.display_name,
            profile_url: user_team.profile_url,
            username: user_team.username,
            user_type: UserType::Team,
        }
    }
}
