# DynamoEnum Derive Macro

The `DynamoEnum` derive macro automatically generates a `FromStr` trait implementation for enums that follow the pattern used in `dynamo_partition.rs`.

## Usage

```rust
use by_macros::DynamoEnum;
use std::str::FromStr;

#[derive(Debug, Clone, DynamoEnum)]
#[dynamo_enum(error = "String")] // Optional: specify custom error type
pub enum Partition {
    User(String),
    Email(String),
    Feed(String),
}
```

## Features

- **Automatic Pattern Matching**: Generates `FromStr` implementation based on `#[strum(to_string = "PREFIX#{0}")]` attributes
- **Custom Error Types**: Use `#[dynamo_enum(error = "YourErrorType")]` to specify custom error types
- **Prefix Extraction**: Automatically extracts prefixes like "USER#", "EMAIL#", etc. from strum attributes
- **Default Error Handling**: Falls back to `String` error type if none specified

## Generated Code

The macro generates code equivalent to:

```rust
impl std::str::FromStr for Partition {
    type Err = String; // Or your custom error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            s if s.starts_with("USER#") => Partition::User(s["USER#".len()..].to_string()),
            s if s.starts_with("EMAIL#") => Partition::Email(s["EMAIL#".len()..].to_string()),
            s if s.starts_with("FEED#") => Partition::Feed(s["FEED#".len()..].to_string()),
            _ => return Err("Invalid Partition: {s}".to_string()), // Or custom error
        })
    }
}
```

## Error Types

### Default (String)
```rust
#[derive(DynamoEnum)]
#[dynamo_enum(error = "String")]
pub enum MyEnum { /* ... */ }
```

### Custom Error Type
```rust
#[derive(DynamoEnum)]
#[dynamo_enum(error = "crate::Error2")]
pub enum MyEnum { /* ... */ }
```

The macro assumes custom error types have an `InvalidPartitionKey(String)` constructor.

## Requirements

- Enum variants must have exactly one `String` field
- Each variant must have a `#[strum(to_string = "PREFIX#{0}")]` attribute
- Must be used alongside `strum_macros::Display`
