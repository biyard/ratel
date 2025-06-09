use bdk::prelude::*;
use validator::Validate;

use crate::BadgeCreateRequest;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/badges", table = space_badges, action = [create(badges = Vec<BadgeCreateRequest>)])]
pub struct SpaceBadge {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    #[api_model(many_to_one = badges)]
    pub badge_id: i64,
}

// https://metadata.ratel.foundation/digital-act/digitalact-bit2.gif
// https://metadata.ratel.foundation/digital-act/digitalact-dog2.gif
// https://metadata.ratel.foundation/digital-act/digitalact-ether2.gif
// https://metadata.ratel.foundation/digital-act/digitalact-kaia2.gif
// https://metadata.ratel.foundation/digital-act/digitalact-sol2.gif
// https://metadata.ratel.foundation/digital-act/digitalact-teth2.gif
