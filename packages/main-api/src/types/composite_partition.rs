use std::{fmt::Display, str::FromStr};

use crate::*;

use serde_with::{DeserializeFromStr, SerializeDisplay};

#[derive(
    Debug,
    Clone,
    SerializeDisplay,
    DeserializeFromStr,
    Default,
    JsonSchema,
    PartialEq,
    Eq,
    OperationIo,
)]
pub struct CompositePartition<T = Partition, S = Partition>(pub T, pub S);

impl<T, S> Display for CompositePartition<T, S>
where
    T: Display,
    S: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}##{}", self.0, self.1)
    }
}

impl<T, S> FromStr for CompositePartition<T, S>
where
    T: FromStr<Err = Error>,
    S: FromStr<Err = Error>,
{
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, "##").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidPartitionKey(
                "invalid composite partition format".to_string(),
            ));
        }
        let part1 = T::from_str(parts[0])?;
        let part2 = S::from_str(parts[1])?;
        Ok(CompositePartition(part1, part2))
    }
}

impl FromStr for CompositePartition<Partition, Partition> {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, "##").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidPartitionKey(
                "invalid composite partition format".to_string(),
            ));
        }
        let part1 = Partition::from_str(parts[0])?;
        let part2 = Partition::from_str(parts[1])?;
        Ok(CompositePartition(part1, part2))
    }
}

impl From<(Partition, Partition)> for CompositePartition {
    fn from(value: (Partition, Partition)) -> Self {
        CompositePartition(value.0, value.1)
    }
}
