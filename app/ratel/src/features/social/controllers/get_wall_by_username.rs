use crate::auth::UserTeam;
use crate::features::social::*;
#[cfg(feature = "server")]
use crate::features::sub_team::models::{SubTeamApplication, SubTeamApplicationStatus};
use crate::social::pages::member::dto::TeamRole;
use crate::{auth::User, posts::models::Team, *};

/// Returns the **applicant team's** username if the viewer is admin/owner
/// of some team that has an in-flight application (Pending / Returned)
/// targeting `wall_team_id`. Returns `None` otherwise. The lookup walks
/// the viewer's UserTeam rows (filtered for admin/owner role) and
/// queries `SubTeamApplication`'s gsi1 (`find_by_applicant`) for each.
/// In practice the list is short — most users belong to 1–2 teams.
#[cfg(feature = "server")]
async fn find_viewer_pending_application_for(
    cli: &aws_sdk_dynamodb::Client,
    viewer_pk: &Partition,
    wall_team_id: &TeamPartition,
) -> Option<String> {
    let wall_team_uuid = wall_team_id.0.clone();

    let sk_prefix = EntityType::UserTeam(String::new()).to_string();
    let opt = UserTeam::opt().sk(sk_prefix).limit(20);
    let (user_teams, _): (Vec<UserTeam>, _) = UserTeam::query(cli, viewer_pk, opt).await.ok()?;

    for ut in user_teams {
        if !ut.role.is_admin_or_owner() {
            continue;
        }
        let applicant_team_pk_str = match &ut.sk {
            EntityType::UserTeam(s) => s.clone(),
            _ => continue,
        };
        let applicant_pk: Partition = match applicant_team_pk_str.parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        let opt = SubTeamApplication::opt().limit(20).scan_index_forward(false);
        let Ok((apps, _)) =
            SubTeamApplication::find_by_applicant(cli, applicant_pk.clone(), opt).await
        else {
            continue;
        };

        // Any application that the applicant might still want to
        // review on the status page — Pending (waiting), Returned
        // (revise & resubmit), Approved (welcome message + post-decision
        // info), or Rejected (rejection reason). Only `Draft` (never
        // submitted) and `Cancelled` are skipped — those have no
        // meaningful status view.
        let matched = apps.iter().any(|a| {
            a.parent_team_id == wall_team_uuid
                && matches!(
                    a.status,
                    SubTeamApplicationStatus::Pending
                        | SubTeamApplicationStatus::Returned
                        | SubTeamApplicationStatus::Approved
                        | SubTeamApplicationStatus::Rejected
                )
        });
        if matched {
            return Some(ut.username.clone());
        }
    }
    None
}

#[get("/api/social/:username", user: OptionalUser)]
pub async fn get_wall_by_username(username: String) -> Result<Wall> {
    use crate::auth::UserFollow;

    let cli = config::get().dynamodb();
    let users = WallUser::find_by_username(cli, username).await?;

    if users.is_empty() {
        Err(SocialError::InvalidUserName)?;
    }

    let user = user.0;

    let is_following = if let Some(u) = &user {
        let target_pk = match &users[0] {
            WallUser::User(u) => &u.pk,
            WallUser::Team(t) => &t.pk,
        };
        let (follower_pk, follower_sk) = UserFollow::follower_keys(target_pk, &u.pk);
        UserFollow::get(cli, follower_pk, Some(follower_sk))
            .await?
            .is_some()
    } else {
        false
    };

    let mut wall = users[0].clone().into();

    match &mut wall {
        Wall::User {
            id,
            can_edit,
            following,
            ..
        } => {
            *following = is_following;
            if let Some(u) = user {
                let pk_id: UserPartition = u.pk.into();
                if &pk_id == id {
                    *can_edit = true;
                }
            }
        }
        Wall::Team {
            id,
            role,
            following,
            viewer_pending_applicant_username,
            ..
        } => {
            *following = is_following;
            if let Some(u) = user {
                let team_pk: Partition = id.clone().into();

                // Member check
                let member =
                    UserTeam::get(cli, &u.pk, Some(EntityType::UserTeam(team_pk.to_string())))
                        .await?;
                if let Some(m) = member {
                    *role = Some(m.role);
                }

                // Pending application check — only fill if viewer is
                // NOT yet a member; otherwise the HUD already routes
                // to the management page and we can skip the query
                // cost. Scans the viewer's admin/owner teams and asks
                // each one's gsi1 (find_by_applicant) for a row whose
                // parent matches this wall team and whose status is
                // still in-flight (Pending / Returned).
                if role.is_none() {
                    *viewer_pending_applicant_username =
                        find_viewer_pending_application_for(cli, &u.pk, id).await;
                }
            }
        }
    }

    Ok(wall)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "user_type")]
pub enum Wall {
    User {
        id: UserPartition,
        display_name: String,
        username: String,
        profile_url: String,
        description: String,
        can_edit: bool,
        following: bool,
        created_at: i64,
    },
    Team {
        id: TeamPartition,
        display_name: String,
        username: String,
        profile_url: String,
        description: String,
        role: Option<TeamRole>,
        following: bool,
        created_at: i64,
        /// Whether the team has opted into the sub-team program. Drives
        /// the visibility of the sub-team HUD icon on the team home.
        #[serde(default)]
        is_parent_eligible: bool,
        /// Minimum applicant team headcount (`0` = no constraint).
        #[serde(default)]
        min_sub_team_members: i32,
        /// Minimum applicant team age in days (`0` = no constraint).
        #[serde(default)]
        min_sub_team_age_days: i32,
        /// If the viewer is admin/owner of some team and that team has
        /// an in-flight (Pending / Returned) application to this wall
        /// team, this is the applicant team's username. The HUD icon
        /// uses it to route to the application status page instead of
        /// the apply page so the applicant can resume from where they
        /// left off.
        #[serde(default)]
        viewer_pending_applicant_username: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity)]
#[serde(untagged)]
#[dynamo(pk_prefix = "USERNAME", index = "gsi2", name = "find_by_username")]
pub enum WallUser {
    User(User),
    Team(Team),
}

impl Into<Wall> for WallUser {
    fn into(self) -> Wall {
        match self {
            WallUser::User(u) => Wall::User {
                id: u.pk.into(),
                display_name: u.display_name,
                username: u.username,
                profile_url: u.profile_url,
                description: u.description,
                can_edit: false,
                following: false,
                created_at: u.created_at,
            },
            WallUser::Team(t) => Wall::Team {
                id: t.pk.into(),
                display_name: t.display_name,
                username: t.username,
                profile_url: t.profile_url,
                description: t.description,
                role: None,
                following: false,
                created_at: t.created_at,
                is_parent_eligible: t.is_parent_eligible,
                min_sub_team_members: t.min_sub_team_members,
                min_sub_team_age_days: t.min_sub_team_age_days,
                viewer_pending_applicant_username: None,
            },
        }
    }
}
