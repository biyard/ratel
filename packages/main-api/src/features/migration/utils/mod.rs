use std::fmt::Display;

use super::*;
use crate::*;

pub async fn tricky_migrate<T: TrickyMigrator, P: Display>(
    cli: &aws_sdk_dynamodb::Client,
    pk: P,
) -> Result<usize> {
    let version = T::version();
    let doc_type = T::doc_type(pk.to_string());
    let current_version = if let Some(v) = get_migration_version(&cli, &doc_type).await? {
        v
    } else {
        return Ok(0);
    };

    if version <= current_version {
        info!(
            "Skipping migration for {:?}, current version: {}, migration version: {}",
            doc_type, current_version, version
        );

        return Ok(0);
    }

    let affected = T::migrate(cli, pk.to_string()).await?;
    info!(
        "Migrated {:?} to version {}, affected records: {}",
        doc_type,
        T::version(),
        affected
    );
    Migration::new(doc_type, version).create(cli).await?;

    Ok(affected)
}

/// get_migration_version returns the current migration version for the given doc_type.
async fn get_migration_version(
    cli: &aws_sdk_dynamodb::Client,
    doc_type: &MigrationDataType,
) -> Result<Option<i32>> {
    let opt = Migration::opt_one();

    let (items, _bookmark) = Migration::query(cli, doc_type, opt).await?;
    let ret = items.first().map(|m| m.sk.parse::<i32>().unwrap_or(0));

    Ok(ret)
}
