use serde::{Deserialize, Serialize};

use super::{Partition, TeamPartition, UserPartition};

/// Represents either a User or Team identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserOrTeam {
    User(UserPartition),
    Team(TeamPartition),
}

impl std::fmt::Display for UserOrTeam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserOrTeam::User(u) => write!(f, "{u}"),
            UserOrTeam::Team(t) => write!(f, "{t}"),
        }
    }
}

impl From<UserPartition> for UserOrTeam {
    fn from(u: UserPartition) -> Self {
        UserOrTeam::User(u)
    }
}

impl From<TeamPartition> for UserOrTeam {
    fn from(t: TeamPartition) -> Self {
        UserOrTeam::Team(t)
    }
}

impl From<Partition> for UserOrTeam {
    fn from(p: Partition) -> Self {
        match p {
            Partition::Team(inner) => UserOrTeam::Team(TeamPartition(inner)),
            Partition::User(inner) => UserOrTeam::User(UserPartition(inner)),
            other => {
                let s = other.to_string();
                if s.starts_with("TEAM#") {
                    UserOrTeam::Team(TeamPartition(
                        s.strip_prefix("TEAM#").unwrap_or(&s).to_string(),
                    ))
                } else {
                    let inner = s.strip_prefix("USER#").unwrap_or(&s).to_string();
                    UserOrTeam::User(UserPartition(inner))
                }
            }
        }
    }
}
