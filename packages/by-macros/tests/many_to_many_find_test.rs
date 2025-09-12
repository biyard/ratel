#[cfg(feature = "server")]
pub type Result<T> = std::result::Result<T, by_types::ApiError<String>>;

#[cfg(feature = "server")]
pub mod many_to_many_find_test {
    #![allow(unused)]
    use super::*;
    use std::time::SystemTime;

    #[cfg(feature = "server")]
    use by_axum::aide;
    use by_macros::api_model;

    #[api_model(base = "/auth/v1", action = [signup(code = String), reset(code = String)], read_action = refresh, table = users_mtm_test)]
    pub struct User {
        #[api_model(primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(auto = [insert])]
        pub created_at: i64,
        #[api_model(auto = [insert, update])]
        pub updated_at: i64,
        #[api_model(action = [signup, login, reset], unique, read_action = [get_user, find_by_email])]
        pub email: String,
        #[api_model(action = [signup, login, reset], read_action = get_user)]
        pub password: String,

        #[api_model(many_to_many = organization_members_mtm_test, foreign_table_name = organizations_mtm_test, foreign_primary_key = org_id, foreign_reference_key = user_id)]
        #[serde(default)]
        pub orgs: Vec<Organization>,
    }
    #[api_model(base = "/organizations/v2", table = organizations_mtm_test)]
    pub struct Organization {
        #[api_model(summary, primary_key, read_action = find_by_id)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(summary)]
        pub name: String,
        #[api_model(many_to_many = organization_members_mtm_test, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = org_id)]
        #[serde(default)]
        pub users: Vec<User>,
    }
    #[api_model(base = "/members/v2", table = organization_members_mtm_test)]
    pub struct OrganizationMember {
        #[api_model(summary, primary_key)]
        pub id: i64,
        #[api_model(summary, auto = [insert])]
        pub created_at: i64,
        #[api_model(summary, auto = [insert, update])]
        pub updated_at: i64,

        #[api_model(many_to_one = users_mtm_test)]
        pub user_id: i64,
        #[api_model(many_to_one = organizations_mtm_test)]
        pub org_id: i64,

        #[api_model(summary, nullable)]
        pub name: String,
        #[api_model(summary)]
        pub contact: Option<String>,
    }

    #[tokio::test]
    async fn test_many_to_many_tests() {
        let pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(
                option_env!("DATABASE_URL")
                    .unwrap_or("postgres://postgres:postgres@localhost:5432/test"),
            )
            .await
            .unwrap();
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let email = format!("{}@example.com", now);
        let password = "password".to_string();
        let org_name = format!("org-{}", now);
        let member_name = format!("member-{}", now);
        let contact = Some("contact".to_string());

        let u = User::get_repository(pool.clone());
        let o = Organization::get_repository(pool.clone());
        let m = OrganizationMember::get_repository(pool.clone());

        u.create_this_table().await;
        o.create_this_table().await;
        m.create_this_table().await;

        u.create_table().await;
        o.create_table().await;
        m.create_table().await;

        let user = u.insert(email.clone(), password.clone()).await.unwrap();
        let org = o.insert(org_name.clone()).await.unwrap();
        let m1 = m
            .insert(user.id, org.id, member_name.clone(), contact.clone())
            .await
            .unwrap();

        let user = u
            .find_one(&UserReadAction::new().find_by_id(user.id))
            .await
            .unwrap();

        assert_eq!(user.orgs.len(), 1);

        let org = o.insert(format!("{}2", org_name.clone())).await.unwrap();
        let m1 = m
            .insert(user.id, org.id, member_name.clone(), contact.clone())
            .await
            .unwrap();

        let user = u
            .find_one(&UserReadAction::new().find_by_id(user.id))
            .await
            .unwrap();

        assert_eq!(user.orgs.len(), 2);
    }
}
