use super::*;
use crate::*;

#[async_trait::async_trait]
pub trait TrickyMigrator {
    /// version should provides the version number of the migration
    /// If the version is lower than or equal to the current version in the database,
    /// the migration will be skipped
    fn version() -> i32;

    fn doc_type(pk: String) -> MigrationDataType;

    /// migrate should returns the number of affected records
    async fn migrate(cli: &aws_sdk_dynamodb::Client, pk: String) -> Result<usize>;
}
