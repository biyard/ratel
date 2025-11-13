use crate::Error;
use crate::{
    models::team::Team,
    types::*,
    utils::{
        aws::{DynamoClient, SesClient},
        time::get_now_timestamp_millis,
    },
};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct UserTeam {
    pub pk: Partition,
    #[dynamo(prefix = "TEAM_PK", index = "gsi1", name = "find_by_team", pk)]
    pub sk: EntityType,

    // NOTE: Sort teams for a user by last_used_at in descending order.
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

    #[allow(unused_variables)]
    pub async fn send_email(ses: &SesClient, team: Team, user_email: String) -> Result<(), Error> {
        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            tracing::warn!("sending email will be skipped for {}", user_email,);
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
            use crate::utils::html::invite_team_html;
            let mut domain = crate::config::get().domain.to_string();
            if domain.contains("localhost") {
                domain = format!("http://{}", domain).to_string();
            } else {
                domain = format!("https://{}", domain).to_string();
            }

            let url = format!("{}/teams/{}/home", domain, team.clone().username);
            tracing::debug!("team url: {:?}", url.clone());

            let html = invite_team_html(team.clone(), url.clone());

            let text = format!(
                "You're invited to join {team}\nOpen: {url}\n",
                team = team.username,
                url = url
            );

            let mut i = 0;
            let subject = format!("[Ratel] Participate your team.");

            while let Err(e) = ses
                .send_mail_html(&user_email, &subject, &html, Some(&text))
                .await
            {
                btracing::notify!(
                    crate::config::get().slack_channel_monitor,
                    &format!("Failed to send email: {:?}", e)
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
