use bdk::prelude::*;
use std::fmt;
use std::str::FromStr;

/// DID Method types supported by the system
/// Currently focused on did:web, but structured for future expansion
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
    Default,
    JsonSchema,
)]
pub enum DidMethod {
    /// did:web - Web-based DID method (RFC: https://w3c-ccg.github.io/did-method-web/)
    /// DIDs are resolved via HTTPS from .well-known/did.json
    #[default]
    Web,

    /// did:key - Self-contained cryptographic DIDs
    /// Reserved for future implementation
    Key,

    /// did:plc - Public Ledger of Credentials (used by Bluesky)
    /// Reserved for future implementation
    Plc,

    /// Custom/other DID methods
    Other(String),
}

impl DidMethod {
    /// Returns the method name as used in DID identifiers
    pub fn method_name(&self) -> &str {
        match self {
            DidMethod::Web => "web",
            DidMethod::Key => "key",
            DidMethod::Plc => "plc",
            DidMethod::Other(name) => name.as_str(),
        }
    }

    /// Parse a DID method from string
    pub fn from_method_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "web" => Ok(DidMethod::Web),
            "key" => Ok(DidMethod::Key),
            "plc" => Ok(DidMethod::Plc),
            other => Ok(DidMethod::Other(other.to_string())),
        }
    }
}

impl fmt::Display for DidMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.method_name())
    }
}

impl FromStr for DidMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_method_str(s)
    }
}
