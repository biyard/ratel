mod attribute_signer;
mod signed_attribute;
mod verifiable_attributes;

pub use attribute_signer::*;
pub use signed_attribute::*;
pub use verifiable_attributes::*;

pub use ssi::dids::*;
pub use ssi::verification_methods;

// Type aliases for compatibility with existing code
pub type DidDocument = ssi::dids::Document;
pub type DidIdentifier = ssi::dids::DIDBuf;
pub use ssi::dids::DIDBuf as DidBuf;
// Note: VerificationMethod and VerificationMethodSet are traits, not structs, so we can't create simple type aliases
// Use ssi::verification_methods::* directly instead
