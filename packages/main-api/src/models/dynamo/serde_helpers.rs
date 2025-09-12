use serde::{Deserialize, Deserializer, Serializer};

use dto::{
    Membership, Theme, UserType, 
    SpaceStatus, SpaceType, PublishingScope, BoosterType,
    FeedStatus, FeedType, UrlType,
};

// Generic helpers for enum<->i64 when the enum has fixed discriminants

pub mod user_type_num {
    use super::*;
    pub fn serialize<S>(v: &UserType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<UserType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            1 => UserType::Individual,
            2 => UserType::Team,
            3 => UserType::Bot,
            99 => UserType::Anonymous,
            _ => UserType::Individual,
        })
    }
}

pub mod membership_num {
    use super::*;
    pub fn serialize<S>(v: &Membership, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<Membership, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            1 => Membership::Free,
            2 => Membership::Paid1,
            3 => Membership::Paid2,
            4 => Membership::Paid3,
            99 => Membership::Admin,
            _ => Membership::Free,
        })
    }
}

pub mod theme_opt_num {
    use super::*;
    pub fn serialize<S>(v: &Option<Theme>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match v {
            Some(t) => s.serialize_some(&(*t as i64)),
            None => s.serialize_none(),
        }
    }
    pub fn deserialize<'de, D>(d: D) -> Result<Option<Theme>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<i64>::deserialize(d)?;
        Ok(match opt {
            Some(1) => Some(Theme::Light),
            Some(2) => Some(Theme::Dark),
            Some(3) => Some(Theme::SystemDefault),
            Some(_) => Some(Theme::Light),
            None => None,
        })
    }
}

pub mod space_type_num {
    use super::*;
    pub fn serialize<S>(v: &SpaceType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<SpaceType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            1 => SpaceType::Legislation,
            2 => SpaceType::Poll,
            3 => SpaceType::Deliberation,
            4 => SpaceType::Nft,
            5 => SpaceType::Commitee,
            6 => SpaceType::SprintLeague,
            7 => SpaceType::Notice,
            8 => SpaceType::Dagit,
            _ => SpaceType::Legislation,
        })
    }
}

pub mod space_status_num {
    use super::*;
    pub fn serialize<S>(v: &SpaceStatus, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<SpaceStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            1 => SpaceStatus::Draft,
            2 => SpaceStatus::InProgress,
            3 => SpaceStatus::Finish,
            _ => SpaceStatus::Draft,
        })
    }
}

pub mod publishing_scope_num {
    use super::*;
    pub fn serialize<S>(v: &PublishingScope, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<PublishingScope, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            1 => PublishingScope::Private,
            2 => PublishingScope::Public,
            _ => PublishingScope::Private,
        })
    }
}

pub mod booster_type_opt_num {
    use super::*;
    pub fn serialize<S>(v: &Option<BoosterType>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match v {
            Some(b) => s.serialize_some(&(*b as i64)),
            None => s.serialize_none(),
        }
    }
    pub fn deserialize<'de, D>(d: D) -> Result<Option<BoosterType>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<i64>::deserialize(d)?;
        Ok(match opt {
            Some(1) => Some(BoosterType::NoBoost),
            Some(2) => Some(BoosterType::X2),
            Some(3) => Some(BoosterType::X10),
            Some(4) => Some(BoosterType::X100),
            Some(_) => Some(BoosterType::NoBoost),
            None => None,
        })
    }
}

pub mod feed_type_num {
    use super::*;
    pub fn serialize<S>(v: &FeedType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<FeedType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            1 => FeedType::Post,
            2 => FeedType::Reply,
            3 => FeedType::Repost,
            4 => FeedType::DocReview,
            _ => FeedType::Post,
        })
    }
}

pub mod url_type_num {
    use super::*;
    pub fn serialize<S>(v: &UrlType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<UrlType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            0 => UrlType::None,
            1 => UrlType::Image,
            _ => UrlType::None,
        })
    }
}

pub mod feed_status_num {
    use super::*;
    pub fn serialize<S>(v: &FeedStatus, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<FeedStatus, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            1 => FeedStatus::Draft,
            2 => FeedStatus::Published,
            _ => FeedStatus::Published,
        })
    }
}

pub mod conversation_type_num {
    use serde::Deserialize;
    use super::*;
    use dto::ConversationType;
    pub fn serialize<S>(v: &ConversationType, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        s.serialize_i64(*v as i64)
    }
    pub fn deserialize<'de, D>(d: D) -> Result<ConversationType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let n = i64::deserialize(d)?;
        Ok(match n {
            0 => ConversationType::Direct,
            1 => ConversationType::Group,
            2 => ConversationType::Channel,
            _ => ConversationType::Direct,
        })
    }
}
