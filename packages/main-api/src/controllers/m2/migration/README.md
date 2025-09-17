# PostgreSQL to DynamoDB User Migration API

This module provides Axum handlers for migrating users from PostgreSQL to DynamoDB.

## Endpoints

### GET /m2/migration/users

Migrate users from PostgreSQL to DynamoDB.

**Query Parameters:**
- `batch_size` (optional): Number of users to migrate in this batch (default: 100, max: 1000)
- `start_user_id` (optional): Starting user ID for batch migration
- `user_id` (optional): Specific user ID to migrate (overrides batch processing)
- `dry_run` (optional): Validate migration without writing to DynamoDB

**Examples:**

```bash
# Migrate a batch of 50 users starting from ID 1000
curl "http://localhost:3000/m2/migration/users?batch_size=50&start_user_id=1000" \
  -H "Authorization: Bearer <token>"

# Migrate a specific user (ID 12345)
curl "http://localhost:3000/m2/migration/users?user_id=12345" \
  -H "Authorization: Bearer <token>"

# Dry run migration for validation
curl "http://localhost:3000/m2/migration/users?batch_size=10&dry_run=true" \
  -H "Authorization: Bearer <token>"
```

**Response:**
```json
{
  "migrated_count": 50,
  "failed_count": 2,
  "failed_user_ids": [1005, 1010],
  "errors": [
    "User 1005: Invalid username format",
    "User 1010: Missing required field"
  ],
  "next_start_user_id": 1051,
  "dry_run": false,
  "processing_time_ms": 2340
}
```

### GET /m2/migration/stats

Get migration statistics.

**Example:**
```bash
curl "http://localhost:3000/m2/migration/stats" \
  -H "Authorization: Bearer <token>"
```

**Response:**
```json
{
  "total_postgres_users": 10000,
  "total_dynamo_users": 8500,
  "pending_migration": 1500,
  "last_migrated_user_id": 8500
}
```

## Usage Workflow

1. **Check Statistics**: Use `/m2/migration/stats` to see current migration status
2. **Dry Run**: Test migration with `dry_run=true` to validate data
3. **Batch Migration**: Migrate users in batches using `batch_size` and `start_user_id`
4. **Monitor Progress**: Use `next_start_user_id` from responses for pagination
5. **Handle Failures**: Retry specific failed users using `user_id` parameter

## Error Handling

The API handles various error scenarios:
- Invalid PostgreSQL user data
- DynamoDB connection/writing failures
- Missing required environment configuration
- Validation errors during conversion

Failed migrations are reported with specific error messages and user IDs for retry.

## Performance Considerations

- Default batch size of 100 provides good balance between performance and error handling
- Maximum batch size of 1000 prevents overwhelming the system
- Processing includes existence checks to avoid duplicate entries
- Statistics endpoint uses efficient counting methods