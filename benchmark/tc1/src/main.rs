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

    let no_users = option_env!("NO_USERS")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(100);

    let mut user_login_data: Vec<LoginData> = vec![];

    for user in generate_test_users(no_users, test_user_password) {
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

        if let Err(e) = api_client
            .add_oracle_to_space(user.user_id, space_id, &admin_login_data.jwt)
            .await
        {
            println!(
                "✗ Failed to add oracle to space for user {}: {:?}",
                user.nickname, e
            );
        }
        println!(
            "✓ User {} added to space with ID: {}",
            user.nickname, space_id
        );
    }

    println!(
        "✓ All users processed successfully {:?}",
        user_login_data.len()
    );

    let artwork_num = option_env!("NO_ARTWORKS")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(5);
    let mut artwork_ids: Vec<i64> = Vec::new();
    for _ in 0..artwork_num {
        let artwork_id = api_client
            .create_artwork(space_id, &admin_login_data.jwt)
            .await?;
        println!("✓ Created artwork with ID: {}", artwork_id);
        artwork_ids.push(artwork_id);
    }

    // Save Admin Token to admin.csv
    {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file_name = option_env!("ADMIN_CSV").unwrap_or("admin.csv");
        let mut file = BufWriter::new(File::create(file_name)?);

        writeln!(file, "adminToken")?;
        // export admin
        writeln!(file, "{}", admin_login_data.jwt.replace('"', "\"\""),)?;
    }

    // Save space ID to spaces.csv
    {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file_name = option_env!("SPACES_CSV").unwrap_or("spaces.csv");
        let mut file = BufWriter::new(File::create(file_name)?);

        writeln!(file, "spaceId")?;
        writeln!(file, "{}", space_id)?;
    }
    // Save user Tokens to tokens.csv
    {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file_name = option_env!("TOKENS_CSV").unwrap_or("tokens.csv");
        let mut file = BufWriter::new(File::create(file_name)?);

        writeln!(file, "userToken")?;
        for user_login in &user_login_data {
            writeln!(file, "{}", user_login.jwt.replace('"', "\"\""))?;
        }
    }
    //Save artwork IDs to artworks.csv
    {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file_name = option_env!("ARTWORKS_CSV").unwrap_or("artworks.csv");
        let mut file = BufWriter::new(File::create(file_name)?);

        writeln!(file, "artworkId")?;
        for artwork_id in artwork_ids {
            writeln!(file, "{}", artwork_id)?;
        }
    }

    Ok(())
}

// Create User

// #[tokio::main]
// async fn main() -> Result<()> {
//     let sign_domain = option_env!("SIGN_DOMAIN").expect("SIGN_DOMAIN must be set");
//     let api_client = ApiClient::new();

//     let test_user_password =
//         option_env!("TEST_USER_PASSWORD").expect("TEST_USER_PASSWORD must be set");

//     let admin = generate_user("TC-Admin", "TCADMIN@example.com", test_user_password);

//     let token = generate_usersig_token(&sign_domain)
//         .map_err(|e| anyhow!("Failed to generate user signature token: {}", e))?;

//     let admin_login_data = LoginData {
//         email: admin.email.clone(),
//         nickname: admin.nickname.clone(),
//         password: admin.password.clone(),
//         usersig: token.clone(),
//         jwt: String::default(),
//         user_id: 0,
//     };

//     match api_client.create_user(&admin, token).await {
//         Ok(admin_user) => {
//             println!(
//                 "✓ Admin user created successfully: {} (ID: {})",
//                 admin_user.nickname, admin_user.id
//             );
//         }
//         Err(e) => {
//             println!("✗ Admin Already Exist: {}", e);
//         }
//     }

//     let (admin_token, _) = api_client
//         .login_user(&admin.email, &admin.password, &admin_login_data.usersig)
//         .await?;

//     println!("✓ Admin login successful {:?}", admin_token);

//     let no_users = option_env!("NO_USERS")
//         .and_then(|s| s.parse::<usize>().ok())
//         .unwrap_or(100);

//     let mut user_tokens: Vec<String> = vec![];

//     for user in generate_test_users(no_users, test_user_password) {
//         let token = generate_usersig_token(&sign_domain)
//             .map_err(|e| anyhow!("Failed to generate user signature token: {}", e))?;
//         let mut user_login = LoginData {
//             email: user.email.clone(),
//             nickname: user.nickname.clone(),
//             password: user.password.clone(),
//             usersig: token.clone(),
//             jwt: String::default(),
//             user_id: 0,
//         };

//         if let Err(_) = api_client.create_user(&user, token.clone()).await {
//             println!("✗ User already exists: {}", user.nickname);
//         }

//         match api_client
//             .login_user(&user.email, &user.password, &user_login.usersig)
//             .await
//         {
//             Ok((token, user_id)) => {
//                 user_login.jwt = token;
//                 user_login.user_id = user_id;
//             }
//             Err(e) => {
//                 println!("✗ Failed to login user: {}", e);
//                 continue;
//             }
//         }
//         user_tokens.push(user_login.jwt.clone());
//     }

//     println!("✓ All users processed successfully {:?}", user_tokens.len());

//     // Save Admin Token to admin.csv
//     {
//         use std::fs::File;
//         use std::io::{BufWriter, Write};

//         let file_name = option_env!("ADMIN_CSV").unwrap_or("admin.csv");
//         let mut file = BufWriter::new(File::create(file_name)?);

//         writeln!(file, "adminToken")?;
//         // export admin
//         writeln!(file, "{}", admin_login_data.jwt.replace('"', "\"\""),)?;
//     }

//     // Save user Tokens to tokens.csv
//     {
//         use std::fs::File;
//         use std::io::{BufWriter, Write};

//         let file_name = option_env!("TOKENS_CSV").unwrap_or("tokens.csv");
//         let mut file = BufWriter::new(File::create(file_name)?);

//         writeln!(file, "userToken")?;
//         for token in &user_tokens {
//             writeln!(file, "{}", token.replace('"', "\"\""))?;
//         }
//     }

//     Ok(())
// }
