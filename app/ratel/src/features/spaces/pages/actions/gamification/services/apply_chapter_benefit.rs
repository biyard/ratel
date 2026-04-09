//! Applies the completion benefit of a chapter to a user.
//!
//! When a chapter is fully cleared, this function inspects the
//! chapter's `completion_benefit` and, for role-upgrade variants,
//! updates the user's space participation record to the new role.

#[cfg(feature = "server")]
use crate::common::*;
#[cfg(feature = "server")]
use crate::features::spaces::pages::actions::gamification::*;

/// Applies the chapter's completion benefit for the given user.
///
/// Returns `Some(SpaceUserRole)` if a role upgrade was applied, or
/// `None` if the benefit was `XpOnly` (no role change).
///
/// **Note**: The V1 implementation logs the intent but does not yet
/// write a role-upgrade to the `SpaceParticipant` record because the
/// participant model does not carry a `role` field today.  Phase 7+
/// will add role storage and implement the actual DynamoDB update here.
#[cfg(feature = "server")]
pub async fn apply_chapter_benefit(
    _cli: &aws_sdk_dynamodb::Client,
    _space_pk: &Partition,
    _user_pk: &Partition,
    chapter: &SpaceChapter,
) -> Result<Option<SpaceUserRole>> {
    match &chapter.completion_benefit {
        ChapterBenefit::XpOnly => Ok(None),
        ChapterBenefit::RoleUpgradeTo(role) | ChapterBenefit::RoleUpgradeAndXp(role) => {
            // TODO(phase-7): Persist the role upgrade to the user's
            // SpaceParticipant record.  For V1 we return the target
            // role so the client can show the overlay, but the server
            // does not yet persist the promotion.
            tracing::info!(
                space_pk = ?_space_pk,
                user_pk = ?_user_pk,
                new_role = ?role,
                "chapter benefit: role upgrade (persistence deferred to Phase 7)"
            );
            Ok(Some(*role))
        }
    }
}
