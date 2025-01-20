use by_axum::{axum::middleware, logger::root};
use by_types::DatabaseConfig;
use controllers::{assets::v1::AssetControllerV1, topic::v1::TopicControllerV1};
use dto::error::ServiceError;
use tokio::net::TcpListener;
use utils::middlewares::authorization_middleware;

mod controllers {
    pub mod patrons {
        pub mod v1;
    }

    pub mod topic {
        pub mod v1;
    }
    pub mod users {
        pub mod v1;
    }
    pub mod assembly_members {
        pub mod m1;
        pub mod v1;
    }
    pub mod assets {
        pub mod v1;
    }
}

pub mod config;
pub mod models;
pub mod utils;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let log = root();
    let conf = config::get();

    let pool = match conf.database {
        // DatabaseConfig::Postgres { url } => {
        //     sqlx::postgres::PgPoolOptions::new()
        //         .max_connections(5)
        //         .connect(url)
        //         .await?;
        // }
        DatabaseConfig::Sqlite { url } => {
            sqlx::sqlite::SqlitePoolOptions::new()
                .max_connections(5)
                .connect(url)
                .await?
        }
        _ => panic!("not supported dynamodb"),
    };

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS ? (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );"#,
    )
    .bind("testing")
    .execute(&pool)
    .await?;

    let app = by_axum::new()
        .nest(
            "/patrons/v1",
            controllers::patrons::v1::PatronControllerV1::route()?,
        )
        .nest("/assets/v1", AssetControllerV1::route()?)
        .nest("/topics/v1", TopicControllerV1::route()?)
        .nest(
            "/users/v1",
            controllers::users::v1::UserControllerV1::route()?,
        )
        .nest(
            "/assembly_members/v1",
            controllers::assembly_members::v1::AssemblyMemberControllerV1::route()?,
        )
        .nest(
            "/assembly_members/m1",
            controllers::assembly_members::m1::AssemblyMemberControllerM1::route()?,
        )
        .layer(middleware::from_fn(authorization_middleware));

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    slog::info!(log, "listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
