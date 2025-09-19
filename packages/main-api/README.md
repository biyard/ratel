# Ratel Main API

## DynamoDB models
### Structures
- At this, `main` is a table name without prefix.
   - `DYNAMO_TABLE_PREFIX` will be set as a prefix of this table name.
   - For example, if you set `DYNAMO_TABLE_PREFIX` to `ratel-local`, the practical table name will be `ratel-local-main`.
- If you indent multiple tables design, you should create separated directory named by `{table_name}`.
- In the table directory, models should be isolated by partition key typed with `Partition` and `EntityType`.
  - Each isolated models should contains `tests.rs` for testing the models.

```
models
 |- dynamo_tables
      |- main
          |- user
               |- mod.rs
               |- ...
               |- tests.rs
          |- email
               |- mod.rs
               |- ...
               |- tests.rs
          |- feed
               |- mod.rs
               |- ...
               |- tests.rs
```


## Implement `/v3` Endpoint

### Please implement 

### Don't use `dto:Error`. Please Use `Error2` instead.

