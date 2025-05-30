#![allow(dead_code)]
use std::collections::HashMap;

use crate::{by_axum::auth::generate_jwt, by_types::Claims};
use dto::*;

pub struct AppClaims<'a>(pub &'a Claims);

impl<'a> AppClaims<'a> {
    pub fn new(claims: &'a Claims) -> Self {
        Self(claims)
    }

    pub fn generate_token(user: &User) -> Result<String> {
        let mut claims = Claims {
            sub: user.id.to_string(),
            custom: HashMap::from([("email".to_string(), user.email.clone())]),
            ..Claims::default()
        };

        generate_jwt(&mut claims).map_err(|e| {
            tracing::error!("Failed to generate JWT: {}", e);
            Error::JWTGenerationFail(e.to_string())
        })
    }

    pub fn get_user_id(&self) -> i64 {
        self.0.sub.parse().unwrap_or_default()
    }

    pub fn get_email(&self) -> String {
        self.0
            .custom
            .get("email")
            .unwrap_or(&"".to_string())
            .to_string()
    }
}
