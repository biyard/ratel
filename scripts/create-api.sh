name=$(echo $1 | sed -r 's/-api$//')
upper_camel_name=$(echo $name | sed -r 's/([a-z])/\U\1/' | sed -r 's/([_-])([a-z])/\U\2/')
snake_name=$(echo "$name" | sed -r 's/-/_/g')
kebab_name=$(echo "$name" | sed -r 's/([A-Z])/-\L\1/g' | sed 's/^-//')

api_dir=packages/${kebab_name}-api
controller=$api_dir/src/controllers/v1/${snake_name}.rs

echo "Generating API into $api_dir"
echo "named to $kebab_name, $snake_name, $upper_camel_name"

cargo new $api_dir
if [ $? -ne 0 ]; then
    echo "Failed to create $api_dir"
    exit 1
fi

mkdir -p $api_dir/src/controllers/v1

echo "Generating Makefile"
cat <<EOT >> $api_dir/Makefile
ENV ?= local
WORKSPACE_ROOT ?= \$(PWD)/../..
AWS_ACCOUNT_ID ?= \$(shell aws sts get-caller-identity --query "Account" --output text)
REGION ?= \$(shell aws configure get region)
COMMIT ?= \$(shell git rev-parse --short HEAD)
VERSION ?= \$(shell toml get Cargo.toml package.version | tr -d \")

ifeq ("\$(ENV)","prod")
	LOG_LEVEL ?= info
	TABLE_NAME = "dagit-prod"
endif

ifeq ("\$(ENV)","dev")
	LOG_LEVEL ?= debug
endif

SECRET_TOKEN ?=
TABLE_NAME ?= "dagit-dev"
LOG_LEVEL ?= debug
AWS_ACCESS_KEY_ID ?= \$(shell aws configure get aws_access_key_id \$(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= \$(shell aws configure get aws_secret_access_key \$(AWS_FLAG))
AWS_REGION ?= \$(shell aws configure get region)

BUILD_ENV ?= ENV=\$(ENV) LOG_LEVEL=\$(LOG_LEVEL) NODE=\$(NODE) TABLE_NAME=\$(TABLE_NAME) AWS_ACCESS_KEY_ID=\$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=\$(AWS_SECRET_ACCESS_KEY) AWS_REGION=\$(AWS_REGION) VERSION=\$(VERSION) COMMIT=\$(COMMIT) SECRET_TOKEN=\$(SECRET_TOKEN)

run:
	\$(BUILD_ENV) cargo watch -x run -w src

build:
	\$(BUILD_ENV) cargo build --release -p \$(SERVICE) --features lambda
EOT


echo "Manipulate Cargo.toml"
cat <<EOT >> $api_dir/Cargo.toml
dto.workspace = true
serde.workspace = true
serde_json.workspace = true
slog.workspace = true
by-axum.workspace = true
easy-dynamodb.workspace = true

tokio = { version = "1.40.0", features = ["full"] }
auth-middleware = { path = "../auth-middleware" }
tower = "0.5.1"

[features]
default = []
lambda = ["by-axum/lambda"]
EOT

echo "Generating main.rs"
cat <<EOT >> $api_dir/src/config.rs
#[derive(Debug)]
pub struct Config {
    pub env: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            env: option_env!("env").expect("You must set ENV").to_string(),
        }
    }
}

static mut CONFIG: Option<Config> = None;

pub fn get() -> &'static Config {
    unsafe {
        if CONFIG.is_none() {
            CONFIG = Some(Config::default());
        }
        &CONFIG.as_ref().unwrap()
    }
}
EOT

echo "Generating main.rs"
rm -rf $api_dir/src/main.rs
cat <<EOT >> $api_dir/src/main.rs
use std::error::Error;

use by_axum::logger::root;
use controllers::v1::${snake_name}::${upper_camel_name}ControllerV1;
use tokio::net::TcpListener;

mod controllers {
    pub mod v1 {
        pub mod ${snake_name};
    }
}

pub mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let log = root();

    easy_dynamodb::init(
        log.clone(),
        option_env!("AWS_ACCESS_KEY_ID")
            .expect("AWS_ACCESS_KEY_ID is required")
            .to_string(),
        option_env!("AWS_SECRET_ACCESS_KEY")
            .expect("AWS_SECRET_ACCESS_KEY is required")
            .to_string(),
        option_env!("AWS_REGION")
            .unwrap_or("ap-northeast-2")
            .to_string(),
        option_env!("TABLE_NAME")
            .expect("TABLE_NAME is required")
            .to_string(),
        "id".to_string(),
        None,
        None,
    );

    let app = by_axum::new()
        .nest("/v1/${kebab_name}", ${upper_camel_name}ControllerV1::route()?);

    let port = option_env!("PORT").unwrap_or("3000");
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    slog::info!(log, "listening on {}", listener.local_addr().unwrap());
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}
EOT


echo "Generating controller"
cat <<EOT >> $controller
use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::{get, post},
        Json,
    },
    log::root,
};
use dto::{common_query_response::CommonQueryResponse, error::ServiceError};
use slog::o;

#[derive(Clone, Debug)]
pub struct ${upper_camel_name}ControllerV1 {
    log: slog::Logger,
}

// NOTE: if already have other pagination, please remove this and use defined one.
#[derive(serde::Deserialize)]
pub struct Pagination {
    page: usize,
    size: usize,
    bookmark: String,
}

#[derive(serde::Deserialize)]
pub struct Create${upper_camel_name}Request {
    name: String,
}

#[derive(serde::Deserialize)]
pub struct Update${upper_camel_name}Request {
    name: Option<String>,
}

// NOTE: This is a real model and recommended to be moved to shared_models
#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct ${upper_camel_name} {
    id: String,
    r#type: String,
    created_at: u64,
    updated_at: u64,
    deleted_at: Option<u64>,

    name: Option<String>,

    // Indexes, if deleted_at is set, all values of indexes must be empty.
    gsi1: String,
    gsi2: String,
}

impl ${upper_camel_name}ControllerV1 {
    pub fn route() -> Result<by_axum::axum::Router, Box<dyn std::error::Error>> {
        let log = root().new(o!("api-controller" => "${upper_camel_name}ControllerV1"));
        let ctrl = ${upper_camel_name}ControllerV1 {
            log,
        };

        Ok(by_axum::axum::Router::new()
           .route(
               "/:id",
               post(Self::create_${snake_name})
                   .get(Self::get_${snake_name})
                   .delete(Self::delete_${snake_name})
                   .put(Self::update_${snake_name}),
           )
            .with_state(ctrl))
           .route("/", get(Self::list_${snake_name}))
            .with_state(ctrl))
            )
    }

    pub async fn create_${snake_name}(
        State(ctrl): State<${upper_camel_name}ControllerV1>,

        Path(id): Path<String>,
        Json(_body): Json<Create${upper_camel_name}Request>,
    ) -> Result<Json<${upper_camel_name}>, ServiceError> {
        Ok(Json(${upper_camel_name}::default()))
    }

    pub async fn update_${snake_name}(
        State(ctrl): State<${upper_camel_name}ControllerV1>,

        Path(id): Path<String>,
        Json(_body): Json<Update${upper_camel_name}Request>
    ) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn get_${snake_name}(
        State(ctrl): State<${upper_camel_name}ControllerV1>,

        Path(id): Path<String>
    ) -> Result<Json<${upper_camel_name}>, ServiceError> {
        Ok(Json(${upper_camel_name}::default()))
    }

    pub async fn delete_${snake_name}(
        State(ctrl): State<${upper_camel_name}ControllerV1>,

        Path(id): Path<String>,
    ) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn list_${snake_name}(
        State(ctrl): State<${upper_camel_name}ControllerV1>,

        Query(pagination): Query<Pagination>
    ) -> Result<Json<CommonQueryResponse<${upper_camel_name}>>, ServiceError> {
        Ok(Json(CommonQueryResponse::default()))
    }
}
EOT
