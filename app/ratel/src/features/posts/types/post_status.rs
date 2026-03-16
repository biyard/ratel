use crate::features::posts::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, SerializeDisplay, Default, DynamoEnum)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum PostStatus {
    #[default]
    Draft,
    Published,
}

impl<'de> serde::Deserialize<'de> for PostStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PostStatusVisitor;

        impl<'de> serde::de::Visitor<'de> for PostStatusVisitor {
            type Value = PostStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a number (1 or 2) or a string")
            }

            fn visit_u64<E: serde::de::Error>(self, value: u64) -> std::result::Result<PostStatus, E> {
                match value {
                    1 => Ok(PostStatus::Draft),
                    2 => Ok(PostStatus::Published),
                    _ => Err(E::custom(format!("invalid PostStatus number: {}", value))),
                }
            }

            fn visit_i64<E: serde::de::Error>(self, value: i64) -> std::result::Result<PostStatus, E> {
                self.visit_u64(value as u64)
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> std::result::Result<PostStatus, E> {
                value.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_any(PostStatusVisitor)
    }
}
