# Ratel Main API

## DynamoDB models
### Structures
- At this, `main` is a table name without prefix.
   - `DYNAMO_TABLE_PREFIX` will be set as a prefix of this table name.
   - For example, if you set `DYNAMO_TABLE_PREFIX` to `ratel-local`, the practical table name will be `ratel-local-main`.
- If you indent multiple tables design, you should create separated directory named by `{table_name}`.
- In the table directory, models should be isolated by partition key typed with `Partition` and `EntityType`.
  - Each isolated models should contains `tests.rs` for testing the models.

```
models
 |- dynamo_tables
      |- main
          |- user
               |- mod.rs
               |- ...
               |- tests.rs
          |- email
               |- mod.rs
               |- ...
               |- tests.rs
          |- feed
               |- mod.rs
               |- ...
               |- tests.rs
```



## About Implementing `/v3` Endpoint

### ***LocalStack Should be started on your local environment***
Your can start localstack with dynamo admin easily with `make run` method.
(it will execute `docker compose up`)

Also, you can stop compose with `make stop` (`docker compose down`)

---

### 1. Don't use `dto:Error`. Please Use `Error2` instead.

### 2. Record all routes in the `route_v3.rs` file.
To avoid conflicts with existing routes and namespaces, all new routes should be separated into this file.

When defining routes, please use aide methods such as `get_with` and `post_with` as shown below.

For v3 routes, you must always **specify the Response type** in `api_docs` macro.
If the response is `Json<SomeStruct>`, be sure to include the `Json` wrapper.

```rust
get_with(
     {HANDLER_FN},
     api_docs!(
          Json<SomeResponse>, //When Json<SomeStruct>,
          "Endpoint Summary",
          "Description"
     ),
)
```

### 3. All logic using `PostgreSQL` must be migrated to `DynamoDB`.
Functions like `extract_user`, `check_perm`, and other features previously relying on Postgres ***should*** now be imported from `dynamo_extractor.rs` or implemented separately for DynamoDB.

### 4. All Handler must have corresponding test code.

### 5. Please make active use of validators.
Utilize validation libraries and mechanisms to ensure request and response data is properly checked and sanitized in your handlers.
```rust
     //utils/validator.rs
     pub fn validate_nickname(name: &str) ...

     //...handler.rs
     #[derive(..., Validate)]
     pub HandlerRequest {
          ...
          #[validate(custom(function = "validate_nickname"))]
          pub nickname: Option<String>
     }
```



## For VS Code Users
### For Testing
When `DYNAMO_TABLE_PREFIX` required Error,
Please add this code to `settings.json`
```json
     "rust-analyzer.runnables.extraEnv": {
    "DYNAMO_TABLE_PREFIX": "ratel-local"
  },
```
### VSCode Snippet
```json
"Axum POST Handler": {
		"prefix": "axum_post_handler",
		"body": [
			"use crate::{AppState, Error2};",
			"use dto::{JsonSchema, aide, schemars};",
			"use dto::by_axum::{",
			"    auth::Authorization,",
			"    axum::{extract::{Path, Json, State}, Extension},",
			"};",
			"use serde::{Deserialize, Serialize};",
			"",
			"#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]",
			"pub struct ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}PathParams {",
			"    $1",
			"}",
			"",
			"#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]",
			"pub struct ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Request {",
			"    $2",
			"}",
			"",
			"#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]",
			"pub struct ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Response {",
			"    $3",
			"}",
			"",
			"pub async fn ${TM_FILENAME_BASE/(.*)/${1:/snakecase}/}_handler(",
			"    State(AppState { dynamo, .. }): State<AppState>,",
			"    Extension(auth): Extension<Option<Authorization>>,",
			"    Path(params): Path<${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}PathParams>,",
			"    Json(req): Json<${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Request>,",
			") -> Result<Json<${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Response>, Error2> {",
			"    $0",
			"    Ok(Json(${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Response::default()))",
			"}"
		],
		"description": "Creates a new Axum POST handler with aide/JsonSchema support."
	},
	"Axum GET Handler": {
		"prefix": "axum_get_handler",
		"body": [
			"use crate::{AppState, Error2};",
			"use dto::{JsonSchema, aide, schemars};",
			"use dto::by_axum::{",
			"    auth::Authorization,",
			"    axum::{extract::{Path, Query, State}, Extension, Json},",
			"};",
			"use serde::{Deserialize, Serialize};",
			"",
			"#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]",
			"pub struct ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}PathParams {",
			"    $1",
			"}",
			"",
			"#[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]",
			"pub struct ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}QueryParams {",
			"    $2",
			"}",
			"",
			"#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]",
			"pub struct ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Response {",
			"    $3",
			"}",
			"",
			"pub async fn ${TM_FILENAME_BASE/(.*)/${1:/snakecase}/}_handler(",
			"    State(AppState { dynamo, .. }): State<AppState>,",
			"    Extension(auth): Extension<Option<Authorization>>,",
			"    Path(path): Path<${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}PathParams>,",
			"    Query(params): Query<${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}QueryParams>,",
			") -> Result<Json<${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Response>, Error2> {",
			"    $0",
			"    Ok(Json(${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}Response::default()))",
			"}"
		],
		"description": "Creates a new Axum GET handler with aide/JsonSchema support."
	},
	"Dynamo Enum": {
		"prefix": "dynamo_enum",
		"body": [
			"use bdk::prelude::*;",
			"",
			"#[derive(",
			"    Default,",
			"    Debug,",
			"    Clone,",
			"    Eq,",
			"    PartialEq,",
			"    serde_with::SerializeDisplay,",
			"    serde_with::DeserializeFromStr,",
			"    DynamoEnum,",
			"    JsonSchema,",
			")]",
			"pub enum ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/} {",
			"    #[default]",
			"    None",
			"",
			"    $0",
			"}"
		],
		"description": "Creates an enum with associated data and common derives based on the filename."
	},
	"Dynamo Repr Enum": {
		"prefix": "dynamo_enum_repr",
		"body": [
			"use bdk::prelude::*;",
			"#[derive(",
			"    Default,",
			"    Debug,",
			"    Clone,",
			"    Copy,",
			"    Eq,",
			"    PartialEq,",
			"    serde_repr::Serialize_repr,",
			"    serde_repr::Deserialize_repr,",
			"    JsonSchema,",
			")]",
			"#[repr($1)]",
			"pub enum ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/} {",
			"    #[default]",
			"    None = 1,",
			"}"
		],
		"description": "Creates a C-like enum with serde_repr based on the filename."
	},
	"Dynamo Model": {
		"prefix": "dynamo_model",
		"body": [
			"use crate::types::*;",
			"use bdk::prelude::*;",
			"use std::collections::HashMap;",
			"",
			"#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]",
			"pub struct ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/} {",
			"    pub pk: Partition,",
			"    pub sk: EntityType,",
			"",
			"    #[dynamo(prefix = \"TS\", index = \"gsi1\", sk)]",
			"    #[dynamo(prefix = \"TS\", index = \"gsi2\", sk)]",
			"    pub created_at: i64,",
			"    pub updated_at: i64,",
			"",
			"    // Add custom fields here",
			"    $0",
			"}",
			"",
			"impl ${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/} {",
			"    pub fn new(",
			"        $1",
			"    ) -> Self {",
			"        let uid = uuid::Uuid::new_v4().to_string();",
			"        let pk = Partition::${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/}(uid);",
			"        let sk = EntityType::${TM_FILENAME_BASE/(.*)/${1:/pascalcase}/};",
			"",
			"        let now = chrono::Utc::now().timestamp_micros();",
			"",
			"        Self {",
			"            pk,",
			"            sk,",
			"            created_at: now,",
			"            updated_at: now,",
			"            ..Default::default()",
			"        }",
			"    }",
			"}",
			"",
		],
		"description": "Creates a struct with common derives based on the filename."
	},
	"Axum Test Handler": {
		"prefix": "axum_test_handler",
		"body": [
			"#[cfg(test)]",
			"pub mod ${TM_FILENAME_BASE/(.*)/${1:/snakecase}/}_tests {",
			"    use crate::tests::{create_app_state, get_auth, create_test_user};",
			"    use super::*;",
			"",
			"    #[tokio::test]",
			"    async fn test_${TM_FILENAME_BASE/(.*)/${1:/snakecase}/}_handler() {",
			"        let app_state = create_app_state();",
			"        let cli = app_state.dynamo.client.clone()",
			"        let user = create_test_user(&cli).await;",
			"        let auth = get_auth(&user);",
			"    }",
			"",
			"}"
		],
		"description": "Creates a Rust test module and handler based on the filename."
	}

```

