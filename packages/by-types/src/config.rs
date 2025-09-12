#[derive(Debug)]
pub struct AwsConfig {
    pub region: &'static str,
    pub access_key_id: &'static str,
    pub secret_access_key: &'static str,
}

impl Default for AwsConfig {
    fn default() -> Self {
        AwsConfig {
            region: option_env!("AWS_REGION").expect("You must set AWS_REGION"),
            access_key_id: option_env!("AWS_ACCESS_KEY_ID")
                .expect("You must set AWS_ACCESS_KEY_ID"),
            secret_access_key: option_env!("AWS_SECRET_ACCESS_KEY")
                .expect("AWS_SECRET_ACCESS_KEY is required"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AuthConfig {
    Jwt {
        secret: &'static str,
        expiration: u64,
    },
}

impl Default for AuthConfig {
    fn default() -> Self {
        let auth_type = option_env!("AUTH_TYPE").unwrap_or_else(|| {
            tracing::warn!("You didn't set AUTH_TYPE and it will be set to jwt by default");
            "jwt"
        });

        if auth_type.to_lowercase() == "jwt" {
            AuthConfig::Jwt {
                secret: option_env!("JWT_SECRET_KEY").expect("You must set JWT_SECRET_KEY"),
                expiration: option_env!("JWT_EXPIRATION")
                    .unwrap_or("3600".into())
                    .parse()
                    .expect("EXPIRATION must be a number"),
            }
        } else {
            panic!("AUTH_TYPE must be jwt");
        }
    }
}

#[derive(Debug)]
pub enum DatabaseConfig {
    DynamoDb {
        aws: AwsConfig,
        table_name: &'static str,
    },
    Postgres {
        url: &'static str,
        pool_size: u32,
    },
    Sqlite {
        url: &'static str,
    },
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        match option_env!("DATABASE_TYPE")
            .expect("You must set DATABASE_TYPE")
            .to_lowercase()
            .as_str()
        {
            "dynamo" | "dynamodb" => DatabaseConfig::DynamoDb {
                aws: AwsConfig::default(),
                table_name: option_env!("TABLE_NAME").expect("You must set TABLE_NAME"),
            },
            "rds" | "postgres" => DatabaseConfig::Postgres {
                url: option_env!("DATABASE_URL").expect("You must set DATABASE_URL"),
                pool_size: option_env!("POOL_SIZE")
                    .unwrap_or("5".into())
                    .parse()
                    .expect("POOL_SIZE must be a number"),
            },
            "sqlite" => DatabaseConfig::Sqlite {
                url: option_env!("DATABASE_URL").expect("You must set DATABASE_URL"),
            },
            _ => panic!("DATABASE_TYPE must be dynamodb or postgres"),
        }
    }
}
