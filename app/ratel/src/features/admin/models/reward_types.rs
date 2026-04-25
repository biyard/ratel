use crate::common::{RewardCondition, RewardPeriod, RewardUserBehavior};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum ConditionType {
    #[default]
    None,
    MaxClaims,
    MaxPoints,
    MaxUserClaims,
    MaxUserPoints,
}

pub trait RewardConditionExt {
    fn from_type_and_value(ct: &ConditionType, value: i64) -> RewardCondition;
    fn condition_type(&self) -> ConditionType;
    fn value(&self) -> Option<i64>;
    fn label(&self) -> String;
}

impl RewardConditionExt for RewardCondition {
    fn from_type_and_value(ct: &ConditionType, value: i64) -> RewardCondition {
        match ct {
            ConditionType::None => RewardCondition::None,
            ConditionType::MaxClaims => RewardCondition::MaxClaims(value),
            ConditionType::MaxPoints => RewardCondition::MaxPoints(value),
            ConditionType::MaxUserClaims => RewardCondition::MaxUserClaims(value),
            ConditionType::MaxUserPoints => RewardCondition::MaxUserPoints(value),
        }
    }

    fn condition_type(&self) -> ConditionType {
        match self {
            RewardCondition::None => ConditionType::None,
            RewardCondition::MaxClaims(_) => ConditionType::MaxClaims,
            RewardCondition::MaxPoints(_) => ConditionType::MaxPoints,
            RewardCondition::MaxUserClaims(_) => ConditionType::MaxUserClaims,
            RewardCondition::MaxUserPoints(_) => ConditionType::MaxUserPoints,
        }
    }

    fn value(&self) -> Option<i64> {
        match self {
            RewardCondition::None => None,
            RewardCondition::MaxClaims(v)
            | RewardCondition::MaxPoints(v)
            | RewardCondition::MaxUserClaims(v)
            | RewardCondition::MaxUserPoints(v) => Some(*v),
        }
    }

    fn label(&self) -> String {
        match self {
            RewardCondition::None => "None".to_string(),
            RewardCondition::MaxClaims(v) => format!("Max Claims: {}", v),
            RewardCondition::MaxPoints(v) => format!("Max Points: {}", v),
            RewardCondition::MaxUserClaims(v) => format!("Max User Claims: {}", v),
            RewardCondition::MaxUserPoints(v) => format!("Max User Points: {}", v),
        }
    }
}

pub trait RewardPeriodExt {
    fn all() -> Vec<RewardPeriod>;
    fn label(&self) -> &'static str;
}

impl RewardPeriodExt for RewardPeriod {
    fn all() -> Vec<RewardPeriod> {
        vec![
            RewardPeriod::Once,
            RewardPeriod::Hourly,
            RewardPeriod::Daily,
            RewardPeriod::Weekly,
            RewardPeriod::Monthly,
            RewardPeriod::Yearly,
            RewardPeriod::Unlimited,
        ]
    }

    fn label(&self) -> &'static str {
        match self {
            RewardPeriod::Once => "Once",
            RewardPeriod::Hourly => "Hourly",
            RewardPeriod::Daily => "Daily",
            RewardPeriod::Weekly => "Weekly",
            RewardPeriod::Monthly => "Monthly",
            RewardPeriod::Yearly => "Yearly",
            RewardPeriod::Unlimited => "Unlimited",
        }
    }
}

pub trait RewardUserBehaviorExt {
    fn all() -> Vec<RewardUserBehavior>;
    fn label(&self) -> &'static str;
}

impl RewardUserBehaviorExt for RewardUserBehavior {
    fn all() -> Vec<RewardUserBehavior> {
        vec![
            RewardUserBehavior::RespondPoll,
            RewardUserBehavior::DiscussionComment,
            RewardUserBehavior::QuizAnswer,
            RewardUserBehavior::Follow,
            RewardUserBehavior::AttendMeet,
        ]
    }

    fn label(&self) -> &'static str {
        match self {
            RewardUserBehavior::RespondPoll => "Poll Response",
            RewardUserBehavior::DiscussionComment => "Discussion Comment",
            RewardUserBehavior::QuizAnswer => "Quiz Answer",
            RewardUserBehavior::Follow => "Follow",
            RewardUserBehavior::AttendMeet => "Attend Meet",
        }
    }
}
