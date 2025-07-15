use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use bdk::prelude::*;
use dto::{sqlx::PgPool, *};
use tokio::time::sleep;

use crate::config;

#[derive(Clone, Debug)]
pub struct SpaceController {
    repo: SpaceRepository,
    pool: PgPool,
}

static INSTANCE: OnceLock<bool> = OnceLock::new();

async fn notify_telegram(payload: TelegramNotificationPayload) -> Result<()> {
    let client = reqwest::Client::new();
    let telegram_notification_url = config::get().telegram_notification_url;
    client
        .post(format!("{}/{}", telegram_notification_url, "notify"))
        .json(&payload)
        .send()
        .await?;
    Ok(())
}

impl SpaceController {
    pub async fn new(pool: PgPool) -> Self {
        let repo = Space::get_repository(pool.clone());

        let ctrl = Self { repo, pool };
        let arc_ctrl = Arc::new(ctrl.clone());
        if INSTANCE.get().is_none() {
            let res = INSTANCE.set(true);
            if let Err(e) = res {
                tracing::error!("Failed to initialize INSTANCE on {e:?}");
            }
            tokio::spawn(async move {
                let _ = tokio::join!(arc_ctrl.finish_spaces(), arc_ctrl.start_spaces());
                sleep(Duration::from_secs(60)).await;
            });
        }

        ctrl
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new().with_state(self.clone()))
    }
    async fn finish_spaces(&self) -> Result<()> {
        let now = chrono::Utc::now().timestamp();

        // Finish Space
        // Now - 60 seconds
        let spaces = Space::query_builder(0)
            .ended_at_between(now - 60, now)
            .status_equals(SpaceStatus::InProgress)
            .query()
            .map(Space::from)
            .fetch_all(&self.pool.clone())
            .await?;
        let mut result = (0, 0);
        for space in spaces {
            let res = self
                .repo
                .update(
                    space.id,
                    SpaceRepositoryUpdateRequest {
                        status: Some(SpaceStatus::Finish),
                        ..Default::default()
                    },
                )
                .await;
            if let Err(e) = res {
                tracing::error!("Failed to update space {}: {:?}", space.id, e);
                result.1 += 1;
                continue;
            } else {
                result.0 += 1;
            }

            match space.space_type {
                SpaceType::SprintLeague => {
                    let ctrl = SprintLeagueController {
                        pool: self.pool.clone(),
                    };
                    if let Err(e) = ctrl.reward(space.id).await {
                        tracing::error!("Failed to give reward for space {}: {:?}", space.id, e);
                        continue;
                    }
                }
                _ => {}
            }
        }
        tracing::info!("Finished spaces: {} success, {} errors", result.0, result.1);

        Ok(())
    }
    async fn start_spaces(&self) -> Result<(i32, i32)> {
        let now = chrono::Utc::now().timestamp();

        // Start Space
        // Now - 60 seconds
        let spaces = Space::query_builder(0)
            .started_at_between(now - 60, now)
            .status_equals(SpaceStatus::Draft)
            .query()
            .map(Space::from)
            .fetch_all(&self.pool.clone())
            .await?;
        let mut result = (0, 0);
        for space in spaces {
            let res = self
                .repo
                .update(
                    space.id,
                    SpaceRepositoryUpdateRequest {
                        status: Some(SpaceStatus::InProgress),
                        ..Default::default()
                    },
                )
                .await;
            if let Err(e) = res {
                tracing::error!("Failed to update space {}: {:?}", space.id, e);
                result.1 += 1;
                continue;
            } else {
                result.0 += 1;
            }
            match space.space_type {
                SpaceType::SprintLeague => {
                    let ctrl = SprintLeagueController {
                        pool: self.pool.clone(),
                    };
                    if let Err(e) = ctrl.notify(space.id).await {
                        tracing::error!("Failed to notify for space {}: {:?}", space.id, e);
                        continue;
                    }
                }
                _ => {}
            }
        }

        Ok(result)
    }
}

pub struct SprintLeagueController {
    pool: PgPool,
}
impl SprintLeagueController {
    pub async fn notify(&self, space_id: i64) -> Result<()> {
        let space = Space::query_builder(0)
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool)
            .await?;
        let sprint_league = SprintLeague::query_builder(0)
            .space_id_equals(space.id)
            .query()
            .map(SprintLeague::from)
            .fetch_one(&self.pool)
            .await?;

        let player_names: Vec<String> = sprint_league.players.into_iter().map(|p| p.name).collect();
        // Notify Telegram
        let payload = TelegramNotificationPayload::SprintLeague(SprintLeaguePayload {
            id: space.id,
            title: space.title.clone().unwrap_or("Space".to_string()),
            description: space.html_contents.clone(),
            started_at: space.started_at.unwrap(),
            ended_at: space.ended_at.unwrap(),
            player_names,
        });
        if let Err(e) = notify_telegram(payload).await {
            tracing::error!("Failed to notify Telegram: {}", e);
        }
        Ok(())
    }
    pub async fn reward(&self, space_id: i64) -> Result<()> {
        let space = Space::query_builder(0)
            .sprint_leagues_builder(
                SprintLeague::query_builder(0).players_builder(SprintLeaguePlayer::query_builder()),
            )
            .id_equals(space_id)
            .query()
            .map(Space::from)
            .fetch_one(&self.pool)
            .await?;
        let sprint_league = space.sprint_leagues.first().ok_or(Error::NotFound)?;
        // TODO: If the number of votes is the same, who will be the winner?
        let max_votes = sprint_league
            .players
            .iter()
            .map(|p| p.total_votes)
            .max()
            .unwrap_or(0);
        let winners: Vec<_> = sprint_league
            .players
            .iter()
            .filter(|p| p.total_votes == max_votes)
            .collect();

        let winner = if winners.len() == 1 {
            winners[0].id
        } else {
            // TODO: Implement tie-breaking logic: earliest vote, random selection, etc.
            // For now, use the first player (deterministic)
            winners[0].id
        };

        let amount = sprint_league.reward_amount;
        let voters = SprintLeagueVote::query_builder()
            .sprint_league_id_equals(sprint_league.id)
            .sprint_league_player_id_equals(winner)
            .query()
            .map(SprintLeagueVote::from)
            .fetch_all(&self.pool)
            .await?;

        let user_reward_repo = UserPoint::get_repository(self.pool.clone());
        for voter in voters {
            if let Some(code) = voter.referral_code {
                let user = User::query_builder()
                    .referral_code_equals(code)
                    .query()
                    .map(User::from)
                    .fetch_one(&self.pool)
                    .await;
                if let Ok(user) = user {
                    if let Err(e) = user_reward_repo
                        .insert(amount, "Sprint League Referral Reward".to_string(), user.id)
                        .await
                    {
                        tracing::error!(
                            "Failed to give referral reward to user {}: {:?} (amount: {})",
                            user.id,
                            e,
                            amount
                        );
                        continue;
                    }
                }
            }
            let res = user_reward_repo
                .insert(amount, "Sprint League Reward".to_string(), voter.user_id)
                .await;
            if let Err(e) = res {
                tracing::error!(
                    "Failed to give reward to user {}: {:?} (amount: {})",
                    voter.user_id,
                    e,
                    amount
                );
                continue;
            }
        }

        Ok(())
    }
}
