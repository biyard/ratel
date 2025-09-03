use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use bdk::prelude::*;
use dto::{sqlx::PgPool, *};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct SpaceController {
    repo: SpaceRepository,
    pool: PgPool,
}

static INSTANCE: OnceLock<bool> = OnceLock::new();

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
                loop {
                    let _ = arc_ctrl.finish_spaces().await;
                    sleep(Duration::from_secs(10)).await;
                }
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
}

pub struct SprintLeagueController {
    pool: PgPool,
}
impl SprintLeagueController {
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
            .map(|p| p.votes)
            .max()
            .unwrap_or(0);
        let winners: Vec<_> = sprint_league
            .players
            .iter()
            .filter(|p| p.votes == max_votes)
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
