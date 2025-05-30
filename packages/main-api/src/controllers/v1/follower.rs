// use bdk::prelude::*;
// use by_axum::axum::{Router, Json, extract::Path};
// use by_axum::axum::routing::{get, post, put, delete};
// use sqlx::PgPool;
// use crate::models::Follower; 

// pub struct FollowerController {
//     pool: PgPool,
// }

// impl FollowerController {
//     pub fn new(pool: PgPool) -> Self {
//         Self { pool }
//     }

//     pub fn route(self) -> Router {
//         Router::new()
//             .route("/", get(Self::list).post(Self::create))
//             .route("/:id", get(Self::get).put(Self::update).delete(Self::delete))
//             .with_state(self.pool)
//     }

//     async fn list(pool: by_axum::axum::extract::State<PgPool>) -> by_axum::Result<Json<Vec<Follower>>> {
//         // Fetch all followers
//         let followers = sqlx::query_as!(Follower, "SELECT * FROM followers")
//             .fetch_all(&*pool)
//             .await?;
//         Ok(Json(followers))
//     }

//     async fn get(Path(id): Path<i64>, pool: by_axum::axum::extract::State<PgPool>) -> by_axum::Result<Json<Follower>> {
//         let follower = sqlx::query_as!(Follower, "SELECT * FROM followers WHERE id = $1", id)
//             .fetch_one(&*pool)
//             .await?;
//         Ok(Json(follower))
//     }

//     async fn create(Json(follower): Json<Follower>, pool: by_axum::axum::extract::State<PgPool>) -> by_axum::Result<Json<Follower>> {
//         // Insert logic here (you may want to use INSERT ... RETURNING *)
//         let inserted = sqlx::query_as!(
//             Follower,
//             "INSERT INTO followers (profile_image, title, description, followed, user_id, created_at, updated_at)
//              VALUES ($1, $2, $3, $4, $5, $6, $7)
//              RETURNING *",
//             follower.profile_image,
//             follower.title,
//             follower.description,
//             follower.followed,
//             follower.user_id,
//             follower.created_at,
//             follower.updated_at,
//         )
//         .fetch_one(&*pool)
//         .await?;
//         Ok(Json(inserted))
//     }

//     async fn update(Path(id): Path<i64>, Json(follower): Json<Follower>, pool: by_axum::axum::extract::State<PgPool>) -> by_axum::Result<Json<Follower>> {
//         let updated = sqlx::query_as!(
//             Follower,
//             "UPDATE followers SET profile_image = $1, title = $2, description = $3, followed = $4, user_id = $5, updated_at = $6 WHERE id = $7 RETURNING *",
//             follower.profile_image,
//             follower.title,
//             follower.description,
//             follower.followed,
//             follower.user_id,
//             follower.updated_at,
//             id,
//         )
//         .fetch_one(&*pool)
//         .await?;
//         Ok(Json(updated))
//     }

//     async fn delete(Path(id): Path<i64>, pool: by_axum::axum::extract::State<PgPool>) -> by_axum::Result<()> {
//         sqlx::query!("DELETE FROM followers WHERE id = $1", id)
//             .execute(&*pool)
//             .await?;
//         Ok(())
//     }
// }