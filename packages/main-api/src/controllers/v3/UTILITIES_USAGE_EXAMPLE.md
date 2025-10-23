# V3 Controller Utilities Usage Guide

This document demonstrates how to use the utility functions in `controllers/v3/mod.rs`.

## Available Utilities

### 1. `extract_state` - Extract DynamoDB Client

Extracts the DynamoDB client from AppState for use in handlers.

**Signature:**
```rust
pub fn extract_state(State(AppState { dynamo, .. }): State<AppState>) -> aws_sdk_dynamodb::Client
```

**Usage Example:**
```rust
use crate::controllers::v3::extract_state;
use axum::extract::State;
use crate::AppState;

async fn my_handler(state: State<AppState>) {
    let cli = extract_state(state);
    // Use cli for DynamoDB operations
    let user = User::get(&cli, pk, sk).await?;
}
```

### 2. `verify_service_admin` - Verify ServiceAdmin Permission

Verifies that a user is authenticated and has ServiceAdmin privileges by querying the ServiceAdmin table.

**Signature:**
```rust
pub async fn verify_service_admin(
    user: Option<User>,
    cli: &aws_sdk_dynamodb::Client,
) -> Result<User, Error2>
```

**Returns:**
- `Ok(User)` - If user is authenticated and is a ServiceAdmin
- `Err(Error2::NoUserFound)` - If no user is authenticated
- `Err(Error2::NoPermission)` - If user is not a ServiceAdmin

**Usage Example:**
```rust
use crate::controllers::v3::verify_service_admin;
use crate::{AppState, Error2, models::user::User};
use aide::NoApi;
use axum::{extract::State, Json};

#[derive(serde::Serialize)]
struct AdminResponse {
    message: String,
}

/// Admin-only endpoint example
async fn admin_only_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
) -> Result<Json<AdminResponse>, Error2> {
    let cli = &dynamo.client;

    // Verify user is a ServiceAdmin
    let _admin_user = verify_service_admin(user, cli).await?;

    // Continue with admin operations
    // Only ServiceAdmins can reach this point

    Ok(Json(AdminResponse {
        message: "Admin operation successful".to_string(),
    }))
}
```

## Complete Handler Example

Here's a complete example of a ServiceAdmin-only endpoint that manages system settings:

```rust
use crate::{
    controllers::v3::verify_service_admin,
    models::user::User,
    AppState, Error2,
};
use aide::NoApi;
use axum::{extract::State, Json};
use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema, aide::OperationIo)]
pub struct UpdateSystemSettingsRequest {
    pub setting_key: String,
    pub setting_value: String,
}

#[derive(Debug, serde::Serialize, schemars::JsonSchema, aide::OperationIo)]
pub struct SystemSettingsResponse {
    pub success: bool,
    pub message: String,
}

/// Update system settings (ServiceAdmin only)
pub async fn update_system_settings_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Json(req): Json<UpdateSystemSettingsRequest>,
) -> Result<Json<SystemSettingsResponse>, Error2> {
    let cli = &dynamo.client;

    // Step 1: Verify user is a ServiceAdmin
    let _admin = verify_service_admin(user, cli).await?;

    // Step 2: Perform admin operations
    // ... your business logic here ...

    Ok(Json(SystemSettingsResponse {
        success: true,
        message: format!("Setting '{}' updated successfully", req.setting_key),
    }))
}
```

## Best Practices

1. **Always verify ServiceAdmin first**: Call `verify_service_admin` at the beginning of your handler to ensure only authorized users can access the endpoint.

2. **Handle errors appropriately**: The function returns specific errors:
   - `Error2::NoUserFound` - User not authenticated (401)
   - `Error2::NoPermission` - User not a ServiceAdmin (403)

3. **Use with NoApi wrapper**: Always extract the user with `NoApi<Option<User>>` to prevent it from appearing in API documentation.

4. **Testing**: Create ServiceAdmin test users in your test setup:
   ```rust
   // In test setup
   let service_admin = ServiceAdmin::new(user.pk.clone());
   service_admin.create(&ddb).await.unwrap();
   ```

## Creating ServiceAdmin Records

To grant ServiceAdmin privileges to a user:

```rust
use crate::{features::srv_admin::models::service_admin::ServiceAdmin, types::Partition};

// Assuming you have a user's partition key
let user_pk = Partition::User("user123".to_string());

// Create ServiceAdmin record
let service_admin = ServiceAdmin::new(user_pk);
service_admin.create(&ddb_client).await?;
```

## Revoking ServiceAdmin Access

To revoke ServiceAdmin privileges:

```rust
use crate::{features::srv_admin::models::service_admin::ServiceAdmin, types::*};

let user_pk = Partition::User("user123".to_string());

ServiceAdmin::delete(
    &ddb_client,
    Partition::ServiceAdmin(user_pk.to_string()),
    Some(EntityType::ServiceAdmin),
).await?;
```
