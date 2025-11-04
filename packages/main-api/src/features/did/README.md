# DID (Decentralized Identifier) Implementation

This module provides a complete implementation of DID resolution and validation, with a focus on the `did:web` method.

## Overview

Decentralized Identifiers (DIDs) are a new type of identifier that enables verifiable, decentralized digital identity. This implementation follows the [W3C DID Core specification](https://www.w3.org/TR/did-core/).

## Architecture

The DID module is structured following the Ratel feature-based architecture:

```
packages/main-api/src/features/did/
├── types/              # Core DID types
│   ├── did_method.rs          # DID method enumeration
│   ├── did_identifier.rs      # DID parsing and identifier structure
│   ├── did_document.rs        # DID Document structure
│   ├── verification_method.rs # Verification method types
│   └── mod.rs
├── dto/                # Data Transfer Objects
│   ├── resolve_did.rs         # Resolution request/response DTOs
│   ├── validation.rs          # Validation DTOs
│   └── mod.rs
├── resolver.rs         # DID resolution implementation
├── service.rs          # High-level DID service layer
└── mod.rs
```

## Supported DID Methods

### did:web (Fully Implemented)

The `did:web` method resolves DIDs using HTTPS and web infrastructure.

**Format:** `did:web:<domain>:<path>`

**Examples:**
- `did:web:example.com` → resolves to `https://example.com/.well-known/did.json`
- `did:web:example.com:user:alice` → resolves to `https://example.com/user/alice/did.json`
- `did:web:example.com%3A3000` → resolves to `https://example.com:3000/.well-known/did.json`

**Specification:** https://w3c-ccg.github.io/did-method-web/

### Future Methods

The architecture is designed to support additional DID methods:
- `did:key` - Self-contained cryptographic DIDs
- `did:plc` - Public Ledger of Credentials (used by Bluesky)

## Core Components

### 1. DID Identifier (`DidIdentifier`)

Parses and validates DID strings:

```rust
use crate::features::did::DidIdentifier;

let did = DidIdentifier::parse("did:web:example.com")?;
assert_eq!(did.method.method_name(), "web");
assert_eq!(did.method_specific_id, "example.com");

// Get the HTTPS URL for resolution
let url = did.web_document_url()?;
// Returns: "https://example.com/.well-known/did.json"
```

### 2. DID Document (`DidDocument`)

Represents a complete DID document following the W3C specification:

```rust
use crate::features::did::types::DidDocument;

let doc: DidDocument = serde_json::from_str(json_str)?;

// Validate the document
doc.validate()?;

// Access verification methods
let auth_methods = doc.authentication_methods();

// Find services
let services = doc.find_services_by_type("LinkedDomains");
```

### 3. DID Resolver (`DidResolver`)

Resolves DIDs to DID documents:

```rust
use crate::features::did::resolver::DidResolver;

let resolver = DidResolver::new()?;
let result = resolver.resolve("did:web:example.com").await?;

println!("Document: {:?}", result.document);
println!("Metadata: {:?}", result.metadata);
```

**Features:**
- Configurable HTTP timeouts
- Response size limits (default 1MB)
- Redirect handling
- Comprehensive error handling

### 4. DID Service (`DidService`)

High-level service layer for DID operations:

```rust
use crate::features::did::service::DidService;

let service = DidService::new()?;

// Resolve a DID
let response = service.resolve_did("did:web:example.com").await?;

// Validate a DID document
let validation = service.validate_did_document(request);

// Parse and validate DID strings
let did = service.parse_did("did:web:example.com")?;

// Check supported methods
assert!(service.is_method_supported("web"));
```

## DID Document Structure

A DID document contains:

### Context
JSON-LD context (required):
```json
{
  "@context": "https://www.w3.org/ns/did/v1"
}
```

### Verification Methods
Cryptographic keys for signing and encryption:
```json
{
  "verificationMethod": [{
    "id": "did:web:example.com#key-1",
    "type": "Ed25519VerificationKey2020",
    "controller": "did:web:example.com",
    "publicKeyMultibase": "z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK"
  }]
}
```

**Supported verification method types:**
- `Ed25519VerificationKey2018`
- `Ed25519VerificationKey2020`
- `EcdsaSecp256k1VerificationKey2019`
- `JsonWebKey2020`
- `Multikey`
- `P256Key2021`
- `X25519KeyAgreementKey2020`
- `RsaVerificationKey2018`

### Verification Relationships

Define how verification methods can be used:

- **authentication** - For proving authentication (e.g., login)
- **assertionMethod** - For issuing credentials
- **keyAgreement** - For key agreement protocols (encryption)
- **capabilityInvocation** - For invoking capabilities
- **capabilityDelegation** - For delegating capabilities

### Services

Service endpoints for interacting with the DID subject:
```json
{
  "service": [{
    "id": "did:web:example.com#messaging",
    "type": "MessagingService",
    "serviceEndpoint": "https://example.com/messaging"
  }]
}
```

## Usage Examples

### Basic Resolution

```rust
use crate::features::did::service::DidService;

#[tokio::main]
async fn main() -> Result<(), String> {
    let service = DidService::new()?;
    
    let response = service.resolve_did("did:web:example.com").await?;
    
    println!("DID: {}", response.did_document.id);
    println!("Verification methods: {}", 
        response.did_document.verification_method_ids().len());
    
    Ok(())
}
```

### Document Validation

```rust
use crate::features::did::{
    service::DidService,
    dto::ValidateDidDocumentRequest,
};

let service = DidService::new()?;

let response = service.validate_did_document(ValidateDidDocumentRequest {
    did_document: doc,
});

if response.valid {
    println!("Document is valid!");
} else {
    println!("Errors: {:?}", response.errors);
    println!("Warnings: {:?}", response.warnings);
}
```

### Custom Resolution Options

```rust
use crate::features::did::{
    service::DidService,
    resolver::ResolutionOptions,
};

let options = ResolutionOptions {
    timeout_secs: 10,
    max_response_size: 512 * 1024, // 512KB
    follow_redirects: false,
};

let service = DidService::with_options(options)?;
```

## Testing

The module includes comprehensive unit tests:

```bash
cd packages/main-api
cargo test features::did
```

**Test coverage includes:**
- DID parsing and validation
- URL construction for did:web
- Document validation
- Verification method validation
- Service validation
- Error handling

## Integration with Ratel

### Future API Endpoints

This implementation provides the foundation for API endpoints:

```
POST /v3/did/resolve
  - Resolve a DID to a DID document

POST /v3/did/validate
  - Validate a DID document structure

POST /v3/did/verify-signature
  - Verify a signature using a DID
```

### Authentication & Authorization

DIDs can be used for:
- Verifiable authentication without passwords
- Cryptographic proof of identity
- Delegated authorization
- Cross-platform identity

## Security Considerations

### HTTPS Requirement

The `did:web` method **requires HTTPS**. This implementation:
- Only resolves via HTTPS (no HTTP fallback)
- Validates SSL/TLS certificates
- Prevents MITM attacks through secure transport

### Response Size Limits

To prevent DoS attacks:
- Default maximum response size: 1MB
- Configurable via `ResolutionOptions`
- Validation performed before parsing

### Timeout Configuration

Prevent hanging requests:
- Default timeout: 30 seconds
- Configurable per resolver instance
- Applied to all HTTP operations

### URL Validation

Strict validation of DID-derived URLs:
- Domain name validation
- Port number encoding support
- Path component validation
- Fragment identifier handling

## Performance Considerations

### Resolution Caching

DID resolution is marked as "expensive" for methods requiring external calls:

```rust
use crate::features::did::resolver::DidResolver;

if DidResolver::is_resolution_expensive("did:web:example.com") {
    // Consider caching the resolved document
}
```

**Recommendation:** Implement a caching layer for resolved DID documents with appropriate TTL.

### Concurrent Resolution

The resolver is thread-safe and can be shared:

```rust
use std::sync::Arc;
use crate::features::did::resolver::DidResolver;

let resolver = Arc::new(DidResolver::new()?);

// Can be cloned and used across async tasks
let resolver_clone = resolver.clone();
tokio::spawn(async move {
    resolver_clone.resolve("did:web:example.com").await
});
```

## Error Handling

All operations return `Result<T, String>` with descriptive error messages:

```rust
match service.resolve_did("did:web:invalid").await {
    Ok(response) => println!("Success: {:?}", response),
    Err(e) => eprintln!("Error: {}", e),
}
```

**Common error scenarios:**
- Invalid DID syntax
- Network failures
- Invalid DID documents
- HTTP errors (404, 500, etc.)
- Timeout errors
- Response size exceeded

## Standards Compliance

This implementation follows:
- [W3C DID Core 1.0](https://www.w3.org/TR/did-core/)
- [did:web Method Specification](https://w3c-ccg.github.io/did-method-web/)
- JSON-LD 1.1
- RFC 3986 (URI syntax)

## References

- [W3C DID Core Specification](https://www.w3.org/TR/did-core/)
- [did:web Method Spec](https://w3c-ccg.github.io/did-method-web/)
- [DID Primer](https://github.com/WebOfTrustInfo/rwot5-boston/blob/master/topics-and-advance-readings/did-primer.md)
- [MetaMask go-did-it](https://github.com/MetaMask/go-did-it) - Reference implementation in Go

## Future Enhancements

### Planned Features

1. **did:key Support** - Self-contained cryptographic DIDs
2. **did:plc Support** - Bluesky's DID method with rotation
3. **Signature Verification** - Verify signatures using DID verification methods
4. **Key Agreement** - Implement ECDH for encrypted communication
5. **Document Caching** - Redis-based caching for resolved documents
6. **Batch Resolution** - Resolve multiple DIDs in parallel
7. **DID Creation** - Generate did:web documents for users
8. **DID Updates** - Rotate keys and update documents
9. **Metrics & Monitoring** - Track resolution performance and errors

### Extension Points

The architecture supports:
- Custom DID methods via `DidMethod::Other(String)`
- Custom verification method types
- Custom service types
- Pluggable resolution strategies

## Contributing

When extending this module:

1. Follow the feature-based architecture pattern
2. Add comprehensive unit tests
3. Update this README with new functionality
4. Follow Rust naming conventions
5. Add inline documentation for public APIs
6. Handle errors explicitly with descriptive messages

## License

Part of the Ratel platform - see root LICENSE file.
