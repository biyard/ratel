# PostgreSQL to DynamoDB Migration Guide

This document provides instructions for migrating data from PostgreSQL to DynamoDB in the Ratel platform.

## Overview

The migration process consists of:
1. **DynamoDB Models** - New data models designed for single-table DynamoDB pattern
2. **ETL Pipeline** - Batch processing system for migrating data
3. **Validation Tools** - Scripts to verify migration integrity
4. **CLI Tools** - Command-line utilities for managing migrations

## Architecture

### DynamoDB Schema Design

The migration uses a single-table design with the following key patterns:

```
Primary Key Structure:
- PK (Partition Key): {ENTITY_TYPE}#{ID}
- SK (Sort Key): METADATA | {RELATIONSHIP_TYPE}#{ID}

Examples:
- USER#123, METADATA           - User profile data
- USER#123, FOLLOWER#456       - User 456 follows User 123
- USER#123, BADGE#789          - User 123 has Badge 789
- SPACE#456, METADATA          - Space profile data
- SPACE#456, MEMBER#123        - User 123 is member of Space 456
- FEED#789, METADATA           - Feed content
- FEED#789, COMMENT#1234567890#999  - Comment 999 on Feed 789
```

### Global Secondary Indexes (GSIs)

1. **type-index**: `type` (PK) + `sk` (SK) - Query by entity type
2. **gsi1-index**: `gsi1_pk` (PK) + `gsi1_sk` (SK) - Custom access patterns
3. **gsi2-index**: `gsi2_pk` (PK) + `gsi2_sk` (SK) - Additional access patterns

## Prerequisites

1. **DynamoDB Table**: Create table using the schema in `scripts/dynamodb-schema.json`
2. **Environment Variables**:
   ```bash
   DATABASE_URL=postgresql://localhost:5432/ratel
   DYNAMODB_TABLE_NAME=ratel-local
   AWS_ENDPOINT_URL_DYNAMODB=http://localhost:8000  # For local DynamoDB
   ```
3. **AWS Credentials**: Configure AWS CLI or environment variables

## Migration Process

### 1. Create DynamoDB Table

```bash
# Local DynamoDB (using docker-compose)
aws dynamodb create-table --cli-input-json file://scripts/dynamodb-schema.json --endpoint-url http://localhost:8000

# Production DynamoDB
aws dynamodb create-table --cli-input-json file://scripts/dynamodb-schema.json
```

### 2. Build Migration Tools

```bash
cd packages/main-api
cargo build --bin migrate_to_dynamo
cargo build --bin validate_migration
```

### 3. Run Migration

```bash
# Full migration
./target/debug/migrate_to_dynamo migrate

# With custom settings
DATABASE_URL=postgresql://user:pass@host:5432/ratel \
DYNAMODB_TABLE_NAME=ratel-prod \
./target/debug/migrate_to_dynamo migrate
```

### 4. Monitor Progress

The migration tool provides real-time progress updates:

```
Starting complete migration from PostgreSQL to DynamoDB
Total records to migrate: 125,000
Processing 1000 users (offset: 0)
Users migration completed: 10,000 records
Processing 500 spaces (offset: 0)
...
```

### 5. Handle Interruptions

If the migration is interrupted:

```bash
# Check saved migration state files
ls migration_state_*.json

# Resume from checkpoint
./target/debug/migrate_to_dynamo resume migration_1234567890
```

### 6. Validate Migration

```bash
# Full validation
./target/debug/validate_migration all

# Specific validations
./target/debug/validate_migration count     # Check record counts
./target/debug/validate_migration users     # Validate user data
./target/debug/validate_migration sample 1000  # Random sample validation
```

## Migration Details

### Data Mapping

#### Users
- **Primary Record**: `USER#{id}` → `METADATA`
- **Followers**: `USER#{id}` → `FOLLOWER#{follower_id}`
- **Following**: `USER#{follower_id}` → `FOLLOWING#{id}` (via GSI1)
- **Badges**: `USER#{id}` → `BADGE#{badge_id}`
- **Industries**: `USER#{id}` → `INDUSTRY#{industry_id}`

#### Spaces
- **Primary Record**: `SPACE#{id}` → `METADATA`
- **Members**: `SPACE#{id}` → `MEMBER#{user_id}`
- **Likes**: `SPACE#{id}` → `LIKE#{user_id}`
- **Owner Lookup**: Via GSI1 `USER#{owner_id}` → `SPACE#{timestamp}`

#### Feeds
- **Primary Record**: `FEED#{id}` → `METADATA`
- **Comments**: `FEED#{parent_id}` → `COMMENT#{timestamp}#{comment_id}`
- **Likes**: `FEED#{id}` → `LIKE#{user_id}`
- **Author Lookup**: Via GSI1 `USER#{author_id}` → `FEED#{timestamp}`

#### Relationships
- **Group Members**: `GROUP#{id}` → `MEMBER#{user_id}`
- **Discussion Participants**: `DISCUSSION#{id}` → `MEMBER#{user_id}`
- **Space Badges**: `SPACE#{id}` → `BADGE#{badge_id}`

### Denormalization Strategy

To optimize queries, frequently accessed related data is denormalized:

- User profiles include `followers_count`, `followings_count`
- Spaces include `owner_nickname`, `industry_name`
- Feeds include `author_nickname`, `industry_name`
- Comments include `author_nickname`, `author_profile_url`

### Batch Processing

- **Batch Size**: 25 items per batch (DynamoDB limit)
- **Retry Logic**: Exponential backoff with 3 max retries
- **Error Handling**: Continue processing with error logging
- **Progress Tracking**: Real-time statistics and checkpointing

## Performance Considerations

### Migration Performance
- **Throughput**: ~1,000-5,000 records/second (depends on record size)
- **Memory Usage**: ~100-500MB for batch processing
- **Concurrent Processing**: Single-threaded for data consistency

### DynamoDB Performance
- **Read Capacity**: Auto-scaling recommended
- **Write Capacity**: Provision higher capacity during migration
- **Hot Partitions**: Avoided through random UUID suffixes where needed

## Troubleshooting

### Common Issues

1. **Connection Errors**
   ```
   Error: Failed to connect to database
   Solution: Check DATABASE_URL and database availability
   ```

2. **DynamoDB Throttling**
   ```
   Error: ProvisionedThroughputExceededException
   Solution: Increase write capacity or use on-demand billing
   ```

3. **Large Record Errors**
   ```
   Error: Item size exceeds 400KB
   Solution: Review file attachments and rich content
   ```

4. **Memory Issues**
   ```
   Error: Out of memory
   Solution: Reduce batch size or increase system memory
   ```

### Debug Commands

```bash
# Check migration status
./target/debug/migrate_to_dynamo status migration_1234567890

# Validate specific records
./target/debug/validate_migration sample 10

# Check DynamoDB table status
aws dynamodb describe-table --table-name ratel-local
```

### Recovery Procedures

1. **Partial Migration Failure**:
   ```bash
   # Resume from last checkpoint
   ./target/debug/migrate_to_dynamo resume migration_1234567890
   ```

2. **Data Corruption**:
   ```bash
   # Re-run validation
   ./target/debug/validate_migration all
   
   # Delete and recreate table if needed
   aws dynamodb delete-table --table-name ratel-local
   aws dynamodb create-table --cli-input-json file://scripts/dynamodb-schema.json
   ```

3. **Performance Issues**:
   ```bash
   # Check table metrics
   aws dynamodb describe-table --table-name ratel-local
   
   # Adjust provisioned capacity
   aws dynamodb update-table --table-name ratel-local --provisioned-throughput ReadCapacityUnits=1000,WriteCapacityUnits=1000
   ```

## Post-Migration Steps

### 1. Update Application Code
- Switch database connections to DynamoDB
- Update query patterns for new access patterns
- Test critical user flows

### 2. Performance Tuning
- Monitor query patterns and optimize GSIs
- Adjust DynamoDB capacity based on usage
- Implement caching where appropriate

### 3. Data Validation
- Run comprehensive validation tests
- Compare business metrics before/after migration
- Monitor error rates and performance metrics

### 4. Rollback Plan
- Keep PostgreSQL data for rollback period
- Document rollback procedures
- Test rollback process in staging environment

## Migration Timeline

For a typical Ratel database:

| Phase | Duration | Description |
|-------|----------|-------------|
| Setup | 30 minutes | Create tables, configure environment |
| Core Migration | 2-6 hours | Migrate users, spaces, feeds |
| Relationships | 1-3 hours | Migrate junction table data |
| Validation | 1-2 hours | Comprehensive validation |
| **Total** | **4-11 hours** | Complete migration process |

*Times vary based on data size and system performance*

## Monitoring and Alerts

### Key Metrics to Monitor
- Migration progress percentage
- Records per second throughput
- Error rate and types
- DynamoDB consumed capacity
- Memory and CPU usage

### Recommended Alerts
- Migration stalled (no progress for 10 minutes)
- High error rate (>5% of records)
- DynamoDB throttling events
- System resource exhaustion

## Support

For migration issues:
1. Check logs for specific error messages
2. Review troubleshooting section above
3. Run validation tools to identify data issues
4. Consult DynamoDB documentation for capacity planning

Migration state files and logs contain detailed information for debugging and support.