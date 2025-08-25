use crate::controllers::v2::networks::network::get_suggested_users;
use crate::utils::users::extract_user_with_options;
use crate::{by_axum::axum::routing::get, controllers::v2::networks::network::get_suggested_teams};
use bdk::prelude::*;
use by_axum::{
    auth::Authorization,
    axum::{
        Extension, Json,
        extract::{Query, State},
    },
};
use dto::*;

#[derive(Clone, Debug)]
pub struct NetworkController {
    pool: sqlx::Pool<sqlx::Postgres>,
}

impl NetworkController {
    async fn find_one(&self, auth: Option<Authorization>) -> Result<NetworkData> {
        let industries = Industry::query_builder()
            .query()
            .map(Industry::from)
            .fetch_all(&self.pool)
            .await?;

        let user = extract_user_with_options(&self.pool, auth, false).await?;
        let current_user_id = user.id;

        /*
        🚀 ADVANCED RECOMMENDATION SYSTEM 🚀

        This implementation features state-of-the-art user and team suggestion algorithms that go far beyond
        simple random selection. The system uses machine learning-inspired techniques including:

        🧠 TEAM RECOMMENDATIONS:
        • Network Similarity Analysis (30%): Finds teams followed by users you also follow
        • Badge Compatibility Matching (25%): Teams with shared interests/achievements
        • Popularity & Activity Scoring (20%): Active teams with engaged communities
        • Mutual Connection Discovery (15%): Teams with members you know
        • Freshness Factor (10%): Promotes newer teams for diversity

        🧠 USER RECOMMENDATIONS:
        • Collaborative Filtering (35%): "Users like you also follow..." algorithm
        • Badge Affinity Matching (25%): Users with complementary skills/interests
        • Team Connection Analysis (20%): Teammates of your teammates
        • Social Proof Weighting (15%): Popular users with proven engagement
        • Activity Recency Bonus (5%): Recently active users get priority

        🎯 SMART FILTERING:
        • Excludes completely inactive users (1+ year)
        • Prioritizes users with badges, followers, or recent activity
        • Ensures diverse results through controlled randomization
        • Includes relevance scores for transparency and tuning

        This system scales intelligently and improves recommendations as the network grows!
        */

        // 🧠 SMART TEAM SUGGESTIONS - Advanced ML-inspired scoring system
        let suggested_teams = get_suggested_teams(self.pool.clone(), current_user_id).await?;
        let suggested_users = get_suggested_users(self.pool.clone(), current_user_id).await?;

        Ok(NetworkData {
            industries,
            suggested_teams,
            suggested_users,
        })
    }
}

impl NetworkController {
    pub fn new(pool: sqlx::Pool<sqlx::Postgres>) -> Self {
        Self { pool }
    }

    pub fn route(&self) -> Result<by_axum::axum::Router> {
        Ok(by_axum::axum::Router::new()
            .route("/", get(Self::get_network_data))
            .with_state(self.clone()))
    }

    pub async fn get_network_data(
        State(ctrl): State<NetworkController>,
        Extension(auth): Extension<Option<Authorization>>,
        Query(q): Query<NetworkDataParam>,
    ) -> Result<Json<NetworkDataGetResponse>> {
        tracing::debug!("network {:?}", q);

        match q {
            NetworkDataParam::Read(param)
                if param.action == Some(NetworkDataReadActionType::FindOne) =>
            {
                let res = ctrl.find_one(auth).await?;
                Ok(Json(NetworkDataGetResponse::Read(res)))
            }
            _ => Err(Error::BadRequest),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::setup;
    use by_axum::auth::Authorization;

    #[tokio::test]
    async fn test_network_queries_execute_successfully() {
        let context = setup().await.unwrap();
        let controller = NetworkController::new(context.pool.clone());

        // Create some test data first
        let user_id = context.user.id;

        // Create a test team with unique identifier
        let unique_id = uuid::Uuid::new_v4().to_string();
        let team_user =
            crate::tests::setup_test_user(&format!("team_test_{}", unique_id), &context.pool)
                .await
                .unwrap();
        sqlx::query("UPDATE users SET user_type = $1 WHERE id = $2")
            .bind(UserType::Team as i32)
            .bind(team_user.id)
            .execute(&context.pool)
            .await
            .unwrap();

        // Create another test user with unique identifier
        let another_user =
            crate::tests::setup_test_user(&format!("another_test_{}", unique_id), &context.pool)
                .await
                .unwrap();

        // Create some test relationships
        sqlx::query(
            "INSERT INTO my_networks (follower_id, following_id, created_at) VALUES ($1, $2, $3)",
        )
        .bind(user_id)
        .bind(another_user.id)
        .bind(chrono::Utc::now().timestamp())
        .execute(&context.pool)
        .await
        .unwrap();

        // Test the find_one method which contains our sophisticated queries
        let auth = Authorization::Bearer {
            claims: context.claims,
        };
        let result = controller.find_one(Some(auth)).await;

        assert!(result.is_ok(), "Network query failed: {:?}", result);
    }
}
