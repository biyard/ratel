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

    let mut user_login_data: Vec<LoginData> = vec![];

    for user in generate_test_users(5, test_user_password) {
        let mut is_created = true;
        let token = generate_usersig_token(&sign_domain)
            .map_err(|e| anyhow!("Failed to generate user signature token: {}", e))?;
        let mut user_login = LoginData {
            email: user.email.clone(),
            nickname: user.nickname.clone(),
            password: user.password.clone(),
            usersig: token.clone(),
            jwt: String::default(),
            user_id: 0,
        };

        if let Err(_) = api_client.create_user(&user, token.clone()).await {
            is_created = false;
        }

        match api_client
            .login_user(&user.email, &user.password, &user_login.usersig)
            .await
        {
            Ok((token, user_id)) => {
                user_login.jwt = token;
                user_login.user_id = user_id;
            }
            Err(e) => {
                println!("✗ Failed to login user: {}", e);
                continue;
            }
        }
        user_login_data.push(user_login);
        let user = user_login_data.last().unwrap();
        if is_created {
            if let Err(e) = api_client
                .add_oracle(user.user_id, space_id, &user.jwt)
                .await
            {
                eprintln!("✗ Failed to add oracle for user {}: {}", user.nickname, e);
            }
            println!(
                "✓ User {} created and added to space with ID: {}",
                user.nickname, space_id
            );
        } else {
            if let Err(e) = api_client
                .add_oracle_to_space(user.user_id, space_id, &admin_login_data.jwt)
                .await
            {
                eprintln!(
                    "✗ Failed to add oracle to space for user {}: {}",
                    user.nickname, e
                );
            }
            println!(
                "✓ User {} already exists, added to space with ID: {}",
                user.nickname, space_id
            );
        }
    }

    println!(
        "✓ All users processed successfully {:?}",
        user_login_data.len()
    );

    let artwork_id = api_client
        .create_artwork(space_id, &admin_login_data.jwt)
        .await?;
    println!("✓ Created artwork with ID: {}", artwork_id);

    for user in &user_login_data {
        api_client.vote(space_id, artwork_id, &user.jwt).await?;
        println!("✓ User {} voted for artwork {}", user.nickname, artwork_id);
    }

    // Save Admin Token to artwork.csv
    {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let mut file = BufWriter::new(File::create("artwork.csv")?);

        writeln!(file, "artworkId")?;
        // export admin
        writeln!(file, "{}", artwork_id)?;
    }

    Ok(())
}
