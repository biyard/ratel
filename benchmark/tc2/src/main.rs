pub use generate_test_case::{
    ApiClient, generate_test_users, generate_user, generate_usersig_token,
};

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    email: String,
    nickname: String,
    password: String,
    usersig: String,
    user_id: i64,
    jwt: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let sign_domain = option_env!("SIGN_DOMAIN").expect("SIGN_DOMAIN must be set");
    let api_client = ApiClient::new();

    let test_user_password =
        option_env!("TEST_USER_PASSWORD").expect("TEST_USER_PASSWORD must be set");

    let admin = generate_user("TC-Admin", "TCADMIN@example.com", test_user_password);

    let token = generate_usersig_token(&sign_domain)
        .map_err(|e| anyhow!("Failed to generate user signature token: {}", e))?;
    let mut admin_login_data = LoginData {
        email: admin.email.clone(),
        nickname: admin.nickname.clone(),
        password: admin.password.clone(),
        usersig: token.clone(),
        jwt: String::default(),
        user_id: 0,
    };
    match api_client.create_user(&admin, token).await {
        Ok(admin_user) => {
            println!(
                "✓ Admin user created successfully: {} (ID: {})",
                admin_user.nickname, admin_user.id
            );
        }
        Err(e) => {
            println!("✗ Admin Already Exist: {}", e);
        }
    }

    let (admin_token, admin_id) = api_client
        .login_user(&admin.email, &admin.password, &admin_login_data.usersig)
        .await?;
    admin_login_data.jwt = admin_token;
    admin_login_data.user_id = admin_id;
    println!("✓ Admin login successful {:?}", admin_login_data.jwt);

    let space_id = api_client
        .create_dagit_space(admin_login_data.user_id, &admin_login_data.jwt)
        .await?;
    println!("✓ Created dagit space with ID: {}", space_id);

    // Save Admin Token to admin.csv
    {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file_name = option_env!("ADMIN_CSV").unwrap_or("admin.csv");
        let mut file = BufWriter::new(File::create(file_name)?);

        writeln!(file, "adminToken,spaceId")?;
        writeln!(
            file,
            "{},{}",
            admin_login_data.jwt.replace('"', "\"\""),
            space_id
        )?;
    }

    Ok(())
}
