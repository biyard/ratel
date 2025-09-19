use super::*;
use crate::types::*;

#[tokio::test]
async fn tests_create_user() {
    let conf = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let cli = aws_sdk_dynamodb::Client::from_conf(conf);
    let now = chrono::Utc::now().timestamp();
    let _expired_at = now + 3600; // 1 hour later
    let email = format!("a+{}@example.com", now);
    let nickname = format!("nickname-{}", now);
    let profile = "http://example.com/profile.png".to_string();
    let username = format!("user{}", now);

    let user = User::new(
        nickname,
        email,
        profile,
        true,
        true,
        UserType::Individual,
        None,
        username,
        "password".to_string(),
    );

    let res = user.create(&cli).await;
    assert!(res.is_ok(), "failed to create user {:?}", res.err());

    let fetched_user = User::get(&cli, user.pk.clone(), Some(user.sk)).await;
    assert!(fetched_user.is_ok());

    let fetched_user = fetched_user.unwrap();
    assert!(fetched_user.is_some());

    let fetched_user = fetched_user.unwrap();
    assert_eq!(fetched_user.email, user.email);
    assert_eq!(fetched_user.display_name, user.display_name);
    assert_eq!(fetched_user.username, user.username);

    // create user principal
    let principal = format!("principal-{}", now);
    let user_principal = UserPrincipal::new(user.pk.clone(), principal.clone());
    let res = user_principal.create(&cli).await;
    assert!(res.is_ok());

    // create user evm address
    let evm_address = format!("0x{:x}", now);
    let user_evm = UserEvmAddress::new(user.pk.clone(), evm_address.clone());
    let res = user_evm.create(&cli).await;
    assert!(res.is_ok());

    // create user referral code
    let referral_code = format!("referral-{}", now);
    let user_referral = UserReferralCode::new(user.pk.clone(), referral_code.clone());
    let res = user_referral.create(&cli).await;
    assert!(res.is_ok());
    // create user phone number
    let phone_number = format!("+1234567890{}", now);
    let user_phone = UserPhoneNumber::new(user.pk.clone(), phone_number.clone());
    let res = user_phone.create(&cli).await;
    assert!(res.is_ok());

    // create user telegram
    let telegram_id = now;
    let telegram_raw = format!("{{\"id\":{}}}", now);
    let user_telegram = UserTelegram::new(user.pk.clone(), telegram_id, telegram_raw.clone());
    let res = user_telegram.create(&cli).await;
    assert!(res.is_ok());

    // query user metadata
    let metadata = UserMetadata::query(&cli, user.pk.clone()).await;
    assert!(
        metadata.is_ok(),
        "failed to query user metadata {:?}",
        metadata.err()
    );
    let metadata = metadata.unwrap();
    assert_eq!(metadata.len(), 6);

    for item in metadata {
        match item {
            UserMetadata::User(u) => {
                assert_eq!(u.email, user.email);
            }
            UserMetadata::UserPrincipal(up) => {
                assert_eq!(up.principal, principal.clone());
            }
            UserMetadata::UserEvmAddress(ue) => {
                assert_eq!(ue.evm_address, evm_address.clone());
            }
            UserMetadata::UserReferralCode(ur) => {
                assert_eq!(ur.referral_code, referral_code.clone());
            }
            UserMetadata::UserPhoneNumber(upn) => {
                assert_eq!(upn.phone_number, phone_number.clone());
            }
            UserMetadata::UserTelegram(ut) => {
                assert_eq!(ut.telegram_id, telegram_id.clone());
                assert_eq!(ut.telegram_raw, telegram_raw);
            }
        }
    }

    let (p, _bookmark) =
        UserPrincipal::find_by_principal(&cli, &principal, UserPrincipalQueryOption::builder())
            .await
            .expect("failed to find by principal");
    assert_eq!(p.len(), 1);
    assert_eq!(p[0].principal, principal, "{:?}", p);

    let (evm, _bookmark) =
        UserEvmAddress::find_by_evm(&cli, &evm_address, UserEvmAddressQueryOption::builder())
            .await
            .expect("failed to find by evm");
    assert_eq!(evm.len(), 1);
    assert_eq!(evm[0].evm_address, evm_address);

    let (referral, _bookmark) = UserReferralCode::find_by_referral_code(
        &cli,
        &referral_code,
        UserReferralCodeQueryOption::builder(),
    )
    .await
    .expect("failed to find by referral code");

    assert_eq!(referral.len(), 1);
    assert_eq!(referral[0].referral_code, referral_code);

    let (phone, _bookmark) = UserPhoneNumber::find_by_phone_number(
        &cli,
        &phone_number,
        UserPhoneNumberQueryOption::builder(),
    )
    .await
    .expect("failed to find by phone number");
    assert_eq!(phone.len(), 1);
    assert_eq!(phone[0].phone_number, phone_number);

    let (telegram, _bookmark) =
        UserTelegram::find_by_telegram_id(&cli, telegram_id, UserTelegramQueryOption::builder())
            .await
            .expect("failed to find by telegram id");
    assert_eq!(telegram.len(), 1);
    assert_eq!(telegram[0].telegram_id, telegram_id);
}

#[tokio::test]
async fn tests_update_user() {
    let conf = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let cli = aws_sdk_dynamodb::Client::from_conf(conf);
    let now = chrono::Utc::now().timestamp();
    let _expired_at = now + 3600; // 1 hour later
    let email = format!("a+{}@example.com", now);
    let nickname = format!("nickname-{}", now);
    let profile = "http://example.com/profile.png".to_string();
    let username = format!("user{}", now);

    let user = User::new(
        nickname,
        email,
        profile,
        true,
        true,
        UserType::Individual,
        None,
        username,
        "password".to_string(),
    );

    let res = user.create(&cli).await;
    assert!(res.is_ok(), "failed to create user {:?}", res.err());

    let fetched_user = User::get(&cli, user.pk.clone(), Some(user.sk.clone())).await;
    assert!(fetched_user.is_ok());

    let fetched_user = fetched_user.unwrap();
    assert!(fetched_user.is_some());

    let fetched_user = fetched_user.unwrap();
    assert_eq!(fetched_user.email, user.email);
    assert_eq!(fetched_user.display_name, user.display_name);
    assert_eq!(fetched_user.username, user.username);
    assert_eq!(fetched_user.followers_count, 0);

    let new_display_name = "Modified name";
    let res = User::updater(fetched_user.pk, fetched_user.sk)
        .with_display_name(new_display_name.to_string())
        .increase_followers_count(1)
        .execute(&cli)
        .await;

    assert!(res.is_ok(), "failed to update");

    let fetched_user = User::get(&cli, user.pk.clone(), Some(user.sk.clone())).await;
    assert!(fetched_user.is_ok());

    let fetched_user = fetched_user.unwrap();
    assert!(fetched_user.is_some());

    let fetched_user = fetched_user.unwrap();
    assert_eq!(fetched_user.email, user.email);
    assert_eq!(fetched_user.display_name, new_display_name);
    assert_eq!(fetched_user.username, user.username);
    assert_eq!(fetched_user.followers_count, 1);

    let res = User::updater(fetched_user.pk, fetched_user.sk)
        .decrease_followers_count(1)
        .execute(&cli)
        .await;

    assert!(res.is_ok(), "failed to update");

    let fetched_user = User::get(&cli, user.pk.clone(), Some(user.sk.clone())).await;
    assert!(fetched_user.is_ok());

    let fetched_user = fetched_user.unwrap();
    assert!(fetched_user.is_some());

    let fetched_user = fetched_user.unwrap();
    assert_eq!(fetched_user.email, user.email);
    assert_eq!(fetched_user.display_name, new_display_name);
    assert_eq!(fetched_user.username, user.username);
    assert_eq!(fetched_user.followers_count, 0);

    let res = User::updater(fetched_user.pk, fetched_user.sk)
        .decrease_followers_count(1)
        .execute(&cli)
        .await;

    assert!(res.is_ok(), "failed to update");

    let fetched_user = User::get(&cli, user.pk.clone(), Some(user.sk.clone())).await;
    assert!(fetched_user.is_ok());

    let fetched_user = fetched_user.unwrap();
    assert!(fetched_user.is_some());

    let fetched_user = fetched_user.unwrap();
    assert_eq!(fetched_user.email, user.email);
    assert_eq!(fetched_user.display_name, new_display_name);
    assert_eq!(fetched_user.username, user.username);
    assert_eq!(fetched_user.followers_count, -1);

    let new_email = format!("mm+{}@example.com", now);

    let res = User::updater(fetched_user.pk, fetched_user.sk)
        .with_email(new_email.clone())
        .execute(&cli)
        .await;

    assert!(res.is_ok(), "failed to update");

    let fetched_user = User::get(&cli, user.pk.clone(), Some(user.sk)).await;
    assert!(fetched_user.is_ok());

    let fetched_user = fetched_user.unwrap();
    assert!(fetched_user.is_some());

    let fetched_user = fetched_user.unwrap();
    assert_eq!(fetched_user.email, new_email);

    let res = User::find_by_email(&cli, new_email.clone(), Default::default()).await;

    assert!(res.is_ok());
    let (users, _bookmark) = res.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].email, new_email);
}

#[tokio::test]
async fn tests_find_user_metamodel() {
    let conf = aws_sdk_dynamodb::Config::builder()
        .credentials_provider(aws_sdk_dynamodb::config::Credentials::new(
            "test", "test", None, None, "dynamo",
        ))
        .region(Some(aws_sdk_dynamodb::config::Region::new("us-east-1")))
        .endpoint_url("http://localhost:4566")
        .behavior_version_latest()
        .build();

    let cli = aws_sdk_dynamodb::Client::from_conf(conf);
    let now = chrono::Utc::now().timestamp();
    let _expired_at = now + 3600; // 1 hour later
    let email = format!("a+{}@example.com", now);
    let nickname = format!("nickname-{}", now);
    let nickname2 = format!("nickname-{}2", now);
    let profile = "http://example.com/profile.png".to_string();
    let username = format!("user{}", now);

    let user = User::new(
        nickname.clone(),
        email.clone(),
        profile.clone(),
        true,
        true,
        UserType::Individual,
        None,
        username.clone(),
        "password".to_string(),
    );

    let res = user.create(&cli).await;
    assert!(res.is_ok(), "failed to create user {:?}", res.err());

    let user = User::new(
        nickname2.clone(),
        email.clone(),
        profile.clone(),
        true,
        true,
        UserType::Individual,
        None,
        username.clone(),
        "password".to_string(),
    );

    let res = user.create(&cli).await;
    assert!(res.is_ok(), "failed to create user {:?}", res.err());

    let users = UserMetadata::find_by_email(&cli, &email, None::<String>).await;
    assert!(users.is_ok(), "failed: {:?}", users);
    let users = users.unwrap();

    assert_eq!(users.len(), 2);

    if let UserMetadata::User(User {
        email: e,
        display_name: d,
        ..
    }) = &users[0]
    {
        assert_eq!(e, &email);
        assert_eq!(d, &nickname);
    } else {
        assert!(false);
    }

    if let UserMetadata::User(User {
        email: e,
        display_name: d,
        ..
    }) = &users[1]
    {
        assert_eq!(e, &email);
        assert_eq!(d, &nickname2);
    } else {
        assert!(false);
    }
}
