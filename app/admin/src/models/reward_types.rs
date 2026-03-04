use crate::*;

#[derive(Debug, Clone, DynamoEnum, SerializeDisplay, DeserializeFromStr, Eq, PartialEq, Default)]
pub enum RewardUserBehavior {
    #[default]
    RespondPoll,
}

impl RewardUserBehavior {
    pub fn action(&self) -> RewardAction {
        match self {
            Self::RespondPoll => RewardAction::Poll,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![Self::RespondPoll]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::RespondPoll => "Poll Response",
        }
    }
}

#[derive(Debug, Clone, DynamoEnum, SerializeDisplay, DeserializeFromStr, Eq, PartialEq, Default)]
pub enum RewardAction {
    #[default]
    Poll,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEnum, Eq, PartialEq)]
pub enum RewardPeriod {
    #[default]
    Once,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    Unlimited,
}

impl RewardPeriod {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Once,
            Self::Hourly,
            Self::Daily,
            Self::Weekly,
            Self::Monthly,
            Self::Yearly,
            Self::Unlimited,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::Once => "Once",
            Self::Hourly => "Hourly",
            Self::Daily => "Daily",
            Self::Weekly => "Weekly",
            Self::Monthly => "Monthly",
            Self::Yearly => "Yearly",
            Self::Unlimited => "Unlimited",
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEnum, Eq, PartialEq)]
pub enum RewardCondition {
    #[default]
    None,
    MaxClaims(i64),
    MaxPoints(i64),
    MaxUserClaims(i64),
    MaxUserPoints(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ConditionType {
    #[default]
    None,
    MaxClaims,
    MaxPoints,
    MaxUserClaims,
    MaxUserPoints,
}

impl ConditionType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::None,
            Self::MaxClaims,
            Self::MaxPoints,
            Self::MaxUserClaims,
            Self::MaxUserPoints,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::MaxClaims => "Max Claims",
            Self::MaxPoints => "Max Points",
            Self::MaxUserClaims => "Max User Claims",
            Self::MaxUserPoints => "Max User Points",
        }
    }
}

impl std::fmt::Display for ConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl RewardCondition {
    pub fn condition_type(&self) -> ConditionType {
        match self {
            Self::None => ConditionType::None,
            Self::MaxClaims(_) => ConditionType::MaxClaims,
            Self::MaxPoints(_) => ConditionType::MaxPoints,
            Self::MaxUserClaims(_) => ConditionType::MaxUserClaims,
            Self::MaxUserPoints(_) => ConditionType::MaxUserPoints,
        }
    }

    pub fn value(&self) -> Option<i64> {
        match self {
            Self::None => Option::None,
            Self::MaxClaims(v)
            | Self::MaxPoints(v)
            | Self::MaxUserClaims(v)
            | Self::MaxUserPoints(v) => Some(*v),
        }
    }

    pub fn from_type_and_value(ct: &ConditionType, value: i64) -> Self {
        match ct {
            ConditionType::None => Self::None,
            ConditionType::MaxClaims => Self::MaxClaims(value),
            ConditionType::MaxPoints => Self::MaxPoints(value),
            ConditionType::MaxUserClaims => Self::MaxUserClaims(value),
            ConditionType::MaxUserPoints => Self::MaxUserPoints(value),
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::None => "None".to_string(),
            Self::MaxClaims(v) => format!("Max Claims: {v}"),
            Self::MaxPoints(v) => format!("Max Points: {v}"),
            Self::MaxUserClaims(v) => format!("Max User Claims: {v}"),
            Self::MaxUserPoints(v) => format!("Max User Points: {v}"),
        }
    }
}
