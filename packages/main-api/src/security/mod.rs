// TODO: After migrate to DYNAMODB, remove these modules

// mod general_permission_verifier;
// mod space_permission_verifier;
// mod team_permission_verifier;

// pub use general_permission_verifier::*;
// use space_permission_verifier::SpacePermissionVerifier;
// pub use team_permission_verifier::*;

// use bdk::prelude::{by_axum::auth::Authorization, *};

// use crate::utils::users::extract_user_with_options;

// pub trait PermissionVerifier {
//     fn has_permission(&self, user: &User, perm: GroupPermission) -> bool;
// }

// pub trait MainGroupPermissionVerifier {
//     fn has_group_permission(&self, perm: GroupPermission) -> bool;
// }

