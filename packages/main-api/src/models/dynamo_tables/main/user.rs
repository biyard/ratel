use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
#[serde(untagged)]
pub enum UserMetadata {
    User(User),
    UserPrincipal(UserPrincipal),
    UserEvmAddress(UserEvmAddress),
    UserReferralCode(UserReferralCode),
    UserPhoneNumber(UserPhoneNumber),
    UserTelegram(UserTelegram),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct User {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    #[dynamo(prefix = "TS", index = "gsi2", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub nickname: String,
    pub profile_url: String,
    #[dynamo(prefix = "EMAIL", name = "find_by_email", index = "gsi1", pk)]
    pub email: String,
    #[dynamo(prefix = "USERNAME", name = "find_by_username", index = "gsi2", pk)]
    pub username: String,

    pub term_agreed: bool,
    pub informed_agreed: bool,

    pub user_type: UserType,
    pub parent_id: Option<String>,

    pub followers_count: i64,
    pub followings_count: i64,

    // profile contents
    pub html_contents: String,
    pub password: String,

    pub membership: Membership,
    pub theme: Theme,
    pub points: i64,
}

impl User {
    pub fn new(
        nickname: String,
        email: String,
        profile_url: String,
        term_agreed: bool,
        informed_agreed: bool,
        user_type: UserType,
        parent_id: Option<String>,
        username: String,
        password: String,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let pk = Partition::User(uid);
        let sk = EntityType::User;

        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            nickname,
            email,
            profile_url,
            term_agreed,
            informed_agreed,
            user_type,
            parent_id,
            username,
            password,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserPrincipal {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_principal", prefix = "PRINCIPAL", index = "gsi1", pk)]
    pub principal: String,
}

impl UserPrincipal {
    pub fn new(pk: Partition, principal: String) -> Self {
        let sk = EntityType::UserPrincipal;

        Self { pk, sk, principal }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserEvmAddress {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_evm", prefix = "EVM", index = "gsi1", pk)]
    pub evm_address: String,
}

impl UserEvmAddress {
    pub fn new(pk: Partition, evm_address: String) -> Self {
        let sk = EntityType::UserEvmAddress;

        Self {
            pk,
            sk,
            evm_address,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserReferralCode {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(
        name = "find_by_referral_code",
        prefix = "REFERRAL",
        index = "gsi1",
        pk
    )]
    pub referral_code: String,
}

impl UserReferralCode {
    pub fn new(pk: Partition, referral_code: String) -> Self {
        let sk = EntityType::UserReferralCode;

        Self {
            pk,
            sk,
            referral_code,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserPhoneNumber {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_phone_number", prefix = "PHONE", index = "gsi1", pk)]
    pub phone_number: String,
}

impl UserPhoneNumber {
    pub fn new(pk: Partition, phone_number: String) -> Self {
        let sk = EntityType::UserPhoneNumber;

        Self {
            pk,
            sk,
            phone_number,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity)]
pub struct UserTelegram {
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(name = "find_by_telegram_id", prefix = "TELEGRAM", index = "gsi1", pk)]
    pub telegram_id: i64,
    pub telegram_raw: String,
}

impl UserTelegram {
    pub fn new(pk: Partition, telegram_id: i64, telegram_raw: String) -> Self {
        let sk = EntityType::UserTelegram;

        Self {
            pk,
            sk,
            telegram_id,
            telegram_raw,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(fetched_user.nickname, user.nickname);
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

        let (telegram, _bookmark) = UserTelegram::find_by_telegram_id(
            &cli,
            telegram_id,
            UserTelegramQueryOption::builder(),
        )
        .await
        .expect("failed to find by telegram id");
        assert_eq!(telegram.len(), 1);
        assert_eq!(telegram[0].telegram_id, telegram_id);
    }
}
