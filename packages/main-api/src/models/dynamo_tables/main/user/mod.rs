// NOTE: User models have been migrated to ratel-auth::models
// - User: common::models::user (canonical struct definition)
// - UserEvmAddress: ratel-auth::models::user_evm_address
// - UserMetadata: ratel-auth::models::user_metadata
// - UserNotification: ratel-auth::models::user_notification
// - UserOAuth: ratel-auth::models::user_oauth
// - UserPhoneNumber: ratel-auth::models::user_phone_number
// - UserPrincipal: ratel-auth::models::user_principal
// - UserReferralCode: ratel-auth::models::user_referral_code
// - UserRefreshToken: ratel-auth::models::user_refresh_token
// - UserRelationship: ratel-auth::models::user_relationship
// - UserTeam: ratel-auth::models::user_team
// - UserTeamGroup: ratel-auth::models::user_team_group
// - UserTelegram: ratel-auth::models::user_telegram
pub mod user;
pub mod user_evm_address;
pub mod user_metadata;
pub mod user_notification;
pub mod user_oauth;
pub mod user_phone_number;
pub mod user_principal;
pub mod user_referral_code;
pub mod user_refresh_token;
pub mod user_relationship;
pub mod user_team;
pub mod user_team_group;
pub mod user_telegram;

pub use user::*;
pub use user_evm_address::*;
pub use user_metadata::*;
pub use user_notification::*;
pub use user_oauth::*;
pub use user_phone_number::*;
pub use user_principal::*;
pub use user_referral_code::*;
pub use user_refresh_token::*;
pub use user_relationship::*;
pub use user_team::*;
pub use user_team_group::*;
pub use user_telegram::*;

#[cfg(test)]
mod tests;
