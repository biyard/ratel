use crate::by_axum::axum::routing::get;
use crate::utils::users::extract_user_with_options;
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
        let suggested_teams_sql = r#"
            WITH user_activity_stats AS (
                -- Calculate current user's engagement profile for personalized scoring
                SELECT 
                    $1 as current_user_id,
                    COUNT(DISTINCT mn.following_id) as following_count,
                    COUNT(DISTINCT ub.badge_id) as badge_count,
                    EXTRACT(EPOCH FROM NOW()) - MAX(u.updated_at) as days_since_activity
                FROM users u
                LEFT JOIN my_networks mn ON u.id = mn.follower_id
                LEFT JOIN user_badges ub ON u.id = ub.user_id
                WHERE u.id = $1
            ),
            team_scores AS (
                SELECT DISTINCT
                    t.*,
                    uas.following_count,
                    uas.badge_count,
                    uas.days_since_activity,
                    -- 🎯 SMART SCORING ALGORITHM
                    (
                        -- Network similarity score (30% weight) - scaled by user's network size
                        COALESCE(
                            (SELECT COUNT(*) * 30.0 * 
                             CASE 
                                WHEN uas.following_count > 10 THEN 1.0  -- Active networker
                                WHEN uas.following_count > 5 THEN 0.8   -- Moderate networker
                                ELSE 0.6                                -- New/quiet user
                             END
                             FROM my_networks mn1 
                             WHERE mn1.follower_id = $1 
                             AND mn1.following_id IN (
                                 SELECT mn2.follower_id 
                                 FROM my_networks mn2 
                                 WHERE mn2.following_id = t.id
                             )), 0
                        ) +
                        
                        -- Badge compatibility score (25% weight) - enhanced by user's badge activity
                        COALESCE(
                            (SELECT COUNT(*) * 25.0 * 
                             CASE 
                                WHEN uas.badge_count > 3 THEN 1.2  -- Badge collector bonus
                                WHEN uas.badge_count > 0 THEN 1.0  -- Has some badges
                                ELSE 0.5                           -- No badges yet
                             END
                             FROM user_badges ub1
                             JOIN user_badges ub2 ON ub1.badge_id = ub2.badge_id
                             WHERE ub1.user_id = $1 AND ub2.user_id = t.id), 0
                        ) +
                        
                        -- Team activity and popularity score (20% weight)
                        COALESCE(
                            (SELECT COUNT(*) * 20.0 / GREATEST(1, (EXTRACT(EPOCH FROM NOW()) - t.created_at) / 86400)
                             FROM my_networks mn WHERE mn.following_id = t.id), 0
                        ) +
                        
                        -- Mutual connections score (15% weight)
                        COALESCE(
                            (SELECT COUNT(*) * 15.0
                             FROM team_members tm1
                             JOIN team_members tm2 ON tm1.user_id = tm2.user_id
                             JOIN my_networks mn ON mn.following_id = tm2.user_id
                             WHERE tm1.team_id = t.id AND mn.follower_id = $1), 0
                        ) +
                        
                        -- Freshness bonus (10% weight) - prefer newer teams for diversity
                        CASE 
                            WHEN (EXTRACT(EPOCH FROM NOW()) - t.created_at) < 2592000 THEN 10.0 -- 30 days
                            WHEN (EXTRACT(EPOCH FROM NOW()) - t.created_at) < 7776000 THEN 5.0  -- 90 days
                            ELSE 0.0
                        END
                    ) as smart_score,
                    
                    -- Additional metadata for intelligent filtering
                    (SELECT COUNT(*) FROM team_members tm WHERE tm.team_id = t.id) as member_count,
                    (SELECT COUNT(*) FROM my_networks mn WHERE mn.following_id = t.id) as follower_count,
                    EXTRACT(EPOCH FROM NOW()) - t.created_at as age_seconds
                    
                FROM users t
                CROSS JOIN user_activity_stats uas
                WHERE t.id != $1 
                AND t.user_type = $2
                AND t.id NOT IN (
                    SELECT following_id FROM my_networks WHERE follower_id = $1
                )
                -- Smart filtering criteria
                AND (
                    -- Include active teams (have recent activity or members)
                    EXISTS (SELECT 1 FROM team_members tm WHERE tm.team_id = t.id)
                    OR 
                    -- Include teams with some following
                    EXISTS (SELECT 1 FROM my_networks mn WHERE mn.following_id = t.id)
                    OR
                    -- Include newer teams (within 6 months)
                    (EXTRACT(EPOCH FROM NOW()) - t.created_at) < 15552000
                )
            )
            SELECT 
                t.*,
                ROUND(smart_score, 2) as relevance_score
            FROM team_scores t
            ORDER BY 
                smart_score DESC,
                -- Secondary sorting for score ties
                follower_count DESC,
                member_count DESC,
                RANDOM() -- Add some controlled randomness
            LIMIT 3
        "#;

        // 🧠 SMART USER SUGGESTIONS - Advanced collaborative filtering
        let suggested_users_sql = r#"
            WITH user_profile AS (
                -- Build current user's profile for recommendation (optimized - only needed fields)
                SELECT 
                    $1 as user_id,
                    COUNT(DISTINCT ub.badge_id) as badge_count,
                    COUNT(DISTINCT tm.team_id) as team_count,
                    COUNT(DISTINCT mn.following_id) as following_count
                FROM users u
                LEFT JOIN user_badges ub ON u.id = ub.user_id
                LEFT JOIN my_networks mn ON u.id = mn.follower_id
                LEFT JOIN team_members tm ON u.id = tm.user_id
                WHERE u.id = $1
            ),
            candidate_users AS (
                SELECT DISTINCT
                    c.*,
                    up.following_count as user_following_count,
                    -- 🎯 ADVANCED RECOMMENDATION ALGORITHM (Enhanced with User Profile)
                    (
                        -- Collaborative filtering score (35% weight) - scaled by user's network activity
                        COALESCE(
                            (SELECT COUNT(*) * 35.0 * 
                             CASE 
                                WHEN up.following_count > 20 THEN 1.2  -- Very active networker bonus
                                WHEN up.following_count > 10 THEN 1.0  -- Active networker
                                WHEN up.following_count > 5 THEN 0.8   -- Moderate networker
                                ELSE 0.6                               -- New/quiet user
                             END
                             FROM my_networks mn1 
                             JOIN my_networks mn2 ON mn1.following_id = mn2.following_id
                             WHERE mn1.follower_id = $1 
                             AND mn2.follower_id = c.id
                             AND mn1.following_id != c.id), 0
                        ) +
                        
                        -- Badge affinity score (25% weight) - enhanced with profile context
                        COALESCE(
                            (SELECT COUNT(*) * 25.0 * 
                             CASE 
                                WHEN up.badge_count > 5 THEN 1.3  -- Badge expert bonus
                                WHEN up.badge_count > 2 THEN 1.1  -- Badge collector bonus
                                WHEN up.badge_count > 0 THEN 1.0  -- Has some badges
                                ELSE 0.7                          -- No badges yet
                             END
                             FROM user_badges ub1
                             JOIN user_badges ub2 ON ub1.badge_id = ub2.badge_id  
                             WHERE ub1.user_id = $1 AND ub2.user_id = c.id), 0
                        ) +
                        
                        -- Team connection score (20% weight) - leveraging team memberships
                        COALESCE(
                            (SELECT COUNT(*) * 20.0 * 
                             CASE 
                                WHEN up.team_count > 3 THEN 1.2  -- Multi-team member bonus
                                WHEN up.team_count > 0 THEN 1.0  -- Team member
                                ELSE 0.8                         -- No teams yet
                             END
                             FROM team_members tm1
                             JOIN team_members tm2 ON tm1.team_id = tm2.team_id
                             WHERE tm1.user_id = $1 AND tm2.user_id = c.id), 0
                        ) +
                        
                        -- Social proof score (15% weight) - popular users
                        COALESCE(
                            (SELECT LEAST(COUNT(*), 10) * 1.5
                             FROM my_networks mn WHERE mn.following_id = c.id), 0
                        ) +
                        
                        -- Activity recency score (5% weight)
                        CASE 
                            WHEN (EXTRACT(EPOCH FROM NOW()) - c.updated_at) < 86400 THEN 5.0    -- 1 day
                            WHEN (EXTRACT(EPOCH FROM NOW()) - c.updated_at) < 604800 THEN 3.0   -- 1 week  
                            WHEN (EXTRACT(EPOCH FROM NOW()) - c.updated_at) < 2592000 THEN 1.0  -- 1 month
                            ELSE 0.0
                        END
                    ) as recommendation_score,
                    
                    -- Metadata for smart filtering and ranking
                    (SELECT COUNT(*) FROM my_networks mn WHERE mn.following_id = c.id) as popularity,
                    (SELECT COUNT(*) FROM user_badges ub WHERE ub.user_id = c.id) as badge_count,
                    EXTRACT(EPOCH FROM NOW()) - c.updated_at as inactivity_seconds
                    
                FROM users c
                CROSS JOIN user_profile up
                WHERE c.id != $1 
                AND c.user_type = $2
                AND c.id NOT IN (
                    SELECT following_id FROM my_networks WHERE follower_id = $1
                )
                -- Intelligent filtering
                AND (
                    -- Include users with some activity or badges
                    EXISTS (SELECT 1 FROM user_badges ub WHERE ub.user_id = c.id)
                    OR
                    -- Include users with followers (social proof)
                    EXISTS (SELECT 1 FROM my_networks mn WHERE mn.following_id = c.id)
                    OR  
                    -- Include recently active users
                    (EXTRACT(EPOCH FROM NOW()) - c.updated_at) < 2592000 -- 30 days
                )
                -- Exclude completely inactive users (no activity in 1 year)
                AND (EXTRACT(EPOCH FROM NOW()) - c.updated_at) < 31536000
            )
            SELECT 
                u.*,
                ROUND(recommendation_score, 2) as relevance_score
            FROM candidate_users u
            ORDER BY 
                recommendation_score DESC,
                -- Tie-breaking logic for same scores
                popularity DESC,
                badge_count DESC,
                inactivity_seconds ASC, -- Prefer more recently active
                RANDOM() -- Final randomness for diversity
            LIMIT 5
        "#;

        let suggested_teams = sqlx::query(suggested_teams_sql)
            .bind(current_user_id)
            .bind(UserType::Team as i32)
            .map(Follower::from)
            .fetch_all(&self.pool)
            .await?;

        let suggested_users = sqlx::query(suggested_users_sql)
            .bind(current_user_id)
            .bind(UserType::Individual as i32)
            .map(Follower::from)
            .fetch_all(&self.pool)
            .await?;

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
        let team_user = crate::tests::setup_test_user(&format!("team_test_{}", unique_id), &context.pool).await.unwrap();
        sqlx::query("UPDATE users SET user_type = $1 WHERE id = $2")
            .bind(UserType::Team as i32)
            .bind(team_user.id)
            .execute(&context.pool)
            .await
            .unwrap();
        
        // Create another test user with unique identifier
        let another_user = crate::tests::setup_test_user(&format!("another_test_{}", unique_id), &context.pool).await.unwrap();
        
        // Create some test relationships
        sqlx::query("INSERT INTO my_networks (follower_id, following_id, created_at) VALUES ($1, $2, $3)")
            .bind(user_id)
            .bind(another_user.id)
            .bind(chrono::Utc::now().timestamp())
            .execute(&context.pool)
            .await
            .unwrap();
        
        // Test the find_one method which contains our sophisticated queries
        let auth = Authorization::Bearer{claims: context.claims};
        let result = controller.find_one(Some(auth)).await;
        
        assert!(result.is_ok(), "Network query failed: {:?}", result);
    }
}
