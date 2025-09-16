# Ratel Main API

## DynamoDB models
### Structures
- At this, `main` is a table name without prefix.
   - `DYNAMO_TABLE_PREFIX` will be set as a prefix of this table name.
   - For example, if you set `DYNAMO_TABLE_PREFIX` to `ratel-local`, the practical table name will be `ratel-local-main`.
- If you indent multiple tables design, you should create separated directory named by `{table_name}`.
- In the table directory, models should be isolated by partition key typed with `Partition` and `EntityType`.

```
models
 |- dynamo_tables
      |- main
          |- user
          |- email
          |- feed
```
