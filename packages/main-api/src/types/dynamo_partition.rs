use crate::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

use serde::{Deserialize, Deserializer, de};
use std::str::FromStr;

use crate::features::membership::MembershipTier;

use super::EntityType;

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    DynamoEnum,
    SubPartition,
    JsonSchema,
    PartialEq,
    Eq,
    OperationIo,
)]
pub enum Partition {
    #[default]
    None,

    User(String),
    Email(String),
    Feed(String),
    PostReply(String), // POST_REPLY#{{post_pk}}
    PostLike(String),
    Session(String),

    // Spaces
    Space(String),
    SurveySpace(String),
    Requirement,
    SpaceTemplate,

    SpacePost(String),
    SpacePostLike(String),

    Discussion(String),
    DiscussionUser(String),

    PanelAttribute,
    PanelParticipant,
    Panels(String),
    Panel(String),
    // Survey(String),
    // SurveyResponse(String),

    // Poll Space
    // PollSpace(String),
    Poll(String),
    SpacePollUserAnswer(String), // user_pk

    // Sprint League
    SprintLeagueVote(String), // user_pk

    Team(String),

    Promotion(String),

    // Membership
    Membership(String),

    // ServiceAdmin
    ServiceAdmin(String),

    // DID
    Did,
    Attributes,
    AttributeCode(String),

    //Telegram Channel
    TelegramChannel,

    // Payment Sub partition
    Purchase, // For user purchases, USER#{user_id}##PURCHASE
    Payment,  // For user payment, USER#{user_id}##PAYMENT
}

impl Partition {
    pub fn to_space_pk(self) -> crate::Result<Partition> {
        match self {
            Partition::Feed(pk) => Ok(Partition::Space(pk)),
            _ => Err(crate::Error::InvalidPartitionKey(
                "Space key can be only extracted from Feed key".to_string(),
            )),
        }
    }

    pub fn to_post_key(self) -> crate::Result<Partition> {
        match self {
            Partition::Space(pk) => Ok(Partition::Feed(pk)),
            _ => Err(crate::Error::InvalidPartitionKey(
                "Post(Feed) key can be only extracted from Space key".to_string(),
            )),
        }
    }

    pub fn to_post_like_key(self) -> crate::Result<Partition> {
        match self {
            Partition::Feed(pk) => Ok(Partition::PostLike(pk)),
            _ => Err(crate::Error::InvalidPartitionKey(
                "PostLike key can be only extracted from Feed key".to_string(),
            )),
        }
    }

    pub fn is_space_key(&self) -> bool {
        matches!(self, Partition::Space(_))
    }

    pub fn to_poll_sk(&self) -> crate::Result<EntityType> {
        match self {
            Partition::Space(pk) => Ok(EntityType::SpacePoll(pk.clone())),
            _ => Err(crate::Error::InvalidPartitionKey(
                "Poll key can be only extracted from Space key".to_string(),
            )),
        }
    }
}

impl From<MembershipTier> for Partition {
    fn from(tier: MembershipTier) -> Self {
        Partition::Membership(tier.to_string())
    }
}

pub fn path_param_string_to_partition<'de, D>(deserializer: D) -> Result<Partition, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let url_decoded = percent_encoding::percent_decode_str(&s)
        .decode_utf8()
        .map_err(|e| de::Error::custom(format!("Invalid percent-encoding: {}", e)))?;
    let url_decoded = url_decoded.into_owned();

    Ok(Partition::from_str(&url_decoded)
        .map_err(|e| de::Error::custom(format!("Invalid Partition: {}", e)))?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::IntoDeserializer;
    use serde::de::value::{Error as ValueError, StringDeserializer};

    #[test]
    fn test_path_param_valid_poll_space() {
        let deserializer: StringDeserializer<ValueError> =
            String::from("FEED%23abc123").into_deserializer();
        let result = path_param_string_to_partition(deserializer);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_path_param_invalid_encoding() {
        let deserializer: StringDeserializer<ValueError> =
            String::from("FEED%ZZ").into_deserializer();
        let result = path_param_string_to_partition(deserializer);
        assert!(result.is_err());
    }

    #[test]
    fn test_sub_partition_user() {
        let user_partition = UserPartition("123".to_string());
        assert_eq!(user_partition.to_string(), "123");

        let partition: Partition = user_partition.into();
        assert_eq!(partition, Partition::User("123".to_string()));
        assert_eq!(partition.to_string(), "USER#123");
    }

    #[test]
    fn test_sub_partition_space() {
        let space_partition = SpacePartition("space456".to_string());
        assert_eq!(space_partition.to_string(), "space456");

        let partition: Partition = space_partition.into();
        assert_eq!(partition, Partition::Space("space456".to_string()));
        assert_eq!(partition.to_string(), "SPACE#space456");
    }

    #[test]
    fn test_sub_partition_from_str() {
        let user_partition = UserPartition::from_str("test123").unwrap();
        assert_eq!(user_partition.0, "test123");
        assert_eq!(user_partition.to_string(), "test123");
    }

    #[test]
    fn test_sub_partition_from_partition() {
        let partition = Partition::User("user789".to_string());
        let user_partition: UserPartition = partition.into();
        assert_eq!(user_partition.0, "user789");
        assert_eq!(user_partition.to_string(), "user789");
    }

    #[test]
    fn test_sub_partition_serialize_user() {
        let user_partition = UserPartition("serialize_test".to_string());
        let serialized = serde_json::to_string(&user_partition).unwrap();
        assert_eq!(serialized, r#""serialize_test""#);
    }

    #[test]
    fn test_sub_partition_deserialize_user() {
        let json = r#""deserialize_test""#;
        let user_partition: UserPartition = serde_json::from_str(json).unwrap();
        assert_eq!(user_partition.0, "deserialize_test");
        assert_eq!(user_partition.to_string(), "deserialize_test");
    }

    #[test]
    fn test_sub_partition_serialize_space() {
        let space_partition = SpacePartition("space_serialize".to_string());
        let serialized = serde_json::to_string(&space_partition).unwrap();
        assert_eq!(serialized, r#""space_serialize""#);
    }

    #[test]
    fn test_sub_partition_deserialize_space() {
        let json = r#""space_deserialize""#;
        let space_partition: SpacePartition = serde_json::from_str(json).unwrap();
        assert_eq!(space_partition.0, "space_deserialize");
        assert_eq!(space_partition.to_string(), "space_deserialize");
    }

    #[test]
    fn test_sub_partition_roundtrip_serialization() {
        let original = UserPartition("roundtrip_test".to_string());
        let serialized = serde_json::to_string(&original).unwrap();
        let deserialized: UserPartition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(original, deserialized);
        assert_eq!(original.0, deserialized.0);
    }

    #[test]
    fn test_sub_partition_complex_id() {
        let complex_id = "user_123_abc-def";
        let user_partition = UserPartition(complex_id.to_string());

        // Test serialization
        let serialized = serde_json::to_string(&user_partition).unwrap();
        assert_eq!(serialized, format!(r#""{}""#, complex_id));

        // Test deserialization
        let deserialized: UserPartition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.0, complex_id);

        // Test conversion to Partition
        let partition: Partition = deserialized.into();
        assert_eq!(partition.to_string(), format!("USER#{}", complex_id));
    }

    #[test]
    fn test_sub_partition_empty_id() {
        let user_partition = UserPartition("".to_string());
        assert_eq!(user_partition.to_string(), "");

        let serialized = serde_json::to_string(&user_partition).unwrap();
        assert_eq!(serialized, r#""""#);

        let deserialized: UserPartition = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.0, "");
    }

    #[test]
    fn test_sub_partition_clone_and_equality() {
        let original = SpacePartition("clone_test".to_string());
        let cloned = original.clone();

        assert_eq!(original, cloned);
        assert_eq!(original.0, cloned.0);
    }

    #[test]
    fn test_sub_partition_debug() {
        let user_partition = UserPartition("debug_test".to_string());
        let debug_output = format!("{:?}", user_partition);
        assert!(debug_output.contains("debug_test"));
    }
}
