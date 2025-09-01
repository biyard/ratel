use crate::Result;
use crate::UserType;
use bdk::prelude::*;
use sqlx::PgPool;

#[api_model(table = users)]
pub struct SuggestedUser {
    #[api_model(primary_key)]
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub user_type: UserType,
    pub nickname: String,
    pub profile_url: String,
    #[api_model(unique)]
    pub email: String,
    pub username: String,
    #[serde(default)]
    pub html_contents: String,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = follower_id, foreign_reference_key = following_id, aggregator = count)]
    #[serde(default)]
    pub followers_count: i64,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = following_id, foreign_reference_key = follower_id, aggregator = exist)]
    #[serde(default)]
    pub followed: bool,
}

impl SuggestedUser {
    pub async fn get_by_followers(pool: &PgPool, user_id: i64) -> Result<Vec<Self>> {
        Ok(SuggestedUser::query_builder(user_id)
            .followed_is_false()
            .followers_count_greater_than(1)
            .order_by_followers_count_desc()
            .limit(10)
            .query()
            .map(Self::from)
            .fetch_all(pool)
            .await?)
    }

    pub async fn get_by_mutual_spaces(pool: &PgPool, user_id: i64) -> Result<Vec<Self>> {
        let overlapping_spaces_sql = r#"
            SELECT DISTINCT
                u.*,
                COUNT(DISTINCT tm2.space_id) as overlapping_spaces_count,
                COALESCE(followers.followers_count, 0) as followers_count
            FROM users u
            JOIN space_members tm2 ON u.id = tm2.user_id
            LEFT JOIN (
                SELECT following_id, COUNT(*) as followers_count
                FROM my_networks
                GROUP BY following_id
            ) followers ON u.id = followers.following_id
            WHERE u.id != $1
            AND u.id NOT IN (
                SELECT following_id FROM my_networks WHERE follower_id = $1
            )
            AND tm2.space_id IN (
                SELECT tm1.space_id
                FROM space_members tm1
                WHERE tm1.user_id = $1
            )
            GROUP BY u.id, followers.followers_count
            ORDER BY overlapping_spaces_count DESC, followers_count DESC
            LIMIT 10
        "#;

        let overlapping_space_users = sqlx::query(overlapping_spaces_sql)
            .bind(user_id)
            .map(Self::from)
            .fetch_all(pool)
            .await?;

        Ok(overlapping_space_users)
    }

    pub async fn get_by_mutual_connection(pool: &PgPool, user_id: i64) -> Result<Vec<Self>> {
        let mutual_connections_sql = r#"
            SELECT DISTINCT
                u.*,
                COUNT(DISTINCT mutual_conn.following_id) as mutual_connections_count
            FROM users u
            JOIN my_networks mn1 ON u.id = mn1.follower_id
            JOIN my_networks mn2 ON mn1.following_id = mn2.following_id AND mn2.follower_id = $1
            JOIN my_networks mutual_conn ON u.id = mutual_conn.follower_id
            WHERE u.id != $1
            AND u.id NOT IN (
                SELECT following_id FROM my_networks WHERE follower_id = $1
            )
            AND mutual_conn.following_id IN (
                SELECT following_id FROM my_networks WHERE follower_id = $1
            )
            GROUP BY u.id
            ORDER BY mutual_connections_count DESC
            LIMIT 5
        "#;

        let mutual_connection_users = sqlx::query(mutual_connections_sql)
            .bind(user_id)
            .map(Self::from)
            .fetch_all(pool)
            .await?;

        Ok(mutual_connection_users)
    }
}
