use bdk::prelude::*;
use dto::{
    Error, FeedQuery, FeedSummary, Follower, Group, NewsQuery, NewsSummary, Promotion,
    PromotionReadAction, Result, User, UserRepositoryUpdateRequest, UserType,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Query, State},
        },
    },
    sqlx::{PgPool, Pool, Postgres},
};

use crate::utils::{
    referal_code::generate_referral_code,
    users::{extract_principal, extract_user_id},
};

use crate::controllers::v1::{feeds, news, promotions};

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct HomeGatewayQuery {
    pub feed_limit: Option<i64>,

    pub news_limit: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, JsonSchema)]
pub struct HomeGatewayResponse {
    pub user_info: Option<User>,
    pub feeds: Vec<FeedSummary>,
    pub promotions: Option<Promotion>,
    pub news: Vec<NewsSummary>,
    pub suggested_users: Vec<Follower>,
}

pub async fn get_home_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(query): Query<HomeGatewayQuery>,
) -> Result<Json<HomeGatewayResponse>> {
    // Set default limits if not provided
    let feed_limit = query.feed_limit.unwrap_or(10) as usize;
    let news_limit = query.news_limit.unwrap_or(3) as usize;

    // Get User Info
    let user_info = match auth.clone() {
        Some(auth_val) => match extract_principal(&pool, Some(auth_val)).await {
            Ok(principal) => get_user_info(Some(principal), pool.clone()).await.ok(),
            Err(_) => None,
        },
        None => None,
    };

    // Get Feeds
    let feeds_obj = feeds::FeedController::new(pool.clone());
    let feeds_data = feeds_obj
        .query(
            auth.clone(),
            FeedQuery {
                size: feed_limit,
                bookmark: Some(String::from("1")),
                ..Default::default()
            },
        )
        .await?
        .items;

    // Get Hot Promotion
    let promotions_obj = promotions::PromotionController::new(pool.clone());
    let promotions_data = promotions_obj
        .hot_promotion(auth.clone(), PromotionReadAction { action: None })
        .await
        .ok();

    // Get News
    let news_obj = news::NewsController::new(pool.clone());
    let news_data = news_obj
        .query(
            auth.clone(),
            NewsQuery {
                size: news_limit,
                bookmark: Some(String::from("1")),
                ..Default::default()
            },
        )
        .await?
        .items;

    // Get Suggested Users
    let suggested_users = if let Some(auth) = auth {
        let user_id = extract_user_id(&pool, Some(auth.clone())).await?;
        get_suggested_users(pool.clone(), user_id).await?
    } else {
        vec![]
    };

    Ok(Json(HomeGatewayResponse {
        user_info,
        feeds: feeds_data,
        promotions: promotions_data,
        news: news_data,
        suggested_users,
    }))
}

pub async fn get_suggested_users(pool: Pool<Postgres>, user_id: i64) -> Result<Vec<Follower>> {
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

    let suggested_users = sqlx::query(suggested_users_sql)
        .bind(user_id)
        .bind(UserType::Individual as i32)
        .map(Follower::from)
        .fetch_all(&pool)
        .await?;

    Ok(suggested_users)
}

pub async fn get_user_info(principal: Option<String>, pool: Pool<Postgres>) -> Result<User> {
    let user = User::query_builder()
        .principal_equals(principal.ok_or(Error::InvalidUser)?)
        .groups_builder(Group::query_builder())
        .user_type_equals(UserType::Individual)
        .query()
        .map(User::from)
        .fetch_one(&pool)
        .await
        .map_err(|_| Error::NotFound)?;

    let user = if user.referral_code.is_empty() {
        let referral_code = generate_referral_code();
        let user_con = User::get_repository(pool.clone());

        user_con
            .update(
                user.id,
                UserRepositoryUpdateRequest::new().with_referral_code(referral_code),
            )
            .await?
    } else {
        user
    };

    Ok(user)
}
