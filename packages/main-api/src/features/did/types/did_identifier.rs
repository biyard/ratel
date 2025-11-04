use super::did_method::DidMethod;
use bdk::prelude::*;

/// A parsed Decentralized Identifier (DID)
/// Format: did:<method>:<method-specific-identifier>
/// Example: did:web:example.com or did:web:example.com:user:alice
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize, JsonSchema)]
pub struct DidIdentifier {
    /// The DID method (e.g., "web", "key", "plc")
    pub method: DidMethod,

    /// The method-specific identifier
    /// For did:web, this is the domain and optional path (e.g., "example.com" or "example.com:user:alice")
    pub method_specific_id: String,

    /// Optional fragment identifier (the part after #)
    pub fragment: Option<String>,
}

impl DidIdentifier {
    /// Parse a DID string into a DidIdentifier
    ///
    /// # Examples
    /// ```
    /// let did = DidIdentifier::parse("did:web:example.com").unwrap();
    /// assert_eq!(did.method, DidMethod::Web);
    /// assert_eq!(did.method_specific_id, "example.com");
    /// ```
    pub fn parse(did_str: &str) -> Result<Self, String> {
        // Check for fragment first
        let (did_without_fragment, fragment) = match did_str.split_once('#') {
            Some((did, frag)) => (did, Some(frag.to_string())),
            None => (did_str, None),
        };

        // Validate DID prefix
        if !did_without_fragment.starts_with("did:") {
            return Err("DID must start with 'did:'".to_string());
        }

        // Split into components
        let parts: Vec<&str> = did_without_fragment.split(':').collect();
        if parts.len() < 3 {
            return Err("DID must have format did:<method>:<method-specific-id>".to_string());
        }

        let method = DidMethod::from_method_str(parts[1])?;
        let method_specific_id = parts[2..].join(":");

        if method_specific_id.is_empty() {
            return Err("Method-specific identifier cannot be empty".to_string());
        }

        Ok(DidIdentifier {
            method,
            method_specific_id,
            fragment,
        })
    }

    /// Convert the DID back to its string representation
    pub fn to_string(&self) -> String {
        let base = format!("did:{}:{}", self.method, self.method_specific_id);
        match &self.fragment {
            Some(frag) => format!("{}#{}", base, frag),
            None => base,
        }
    }

    /// Get the DID without the fragment
    pub fn to_string_without_fragment(&self) -> String {
        format!("did:{}:{}", self.method, self.method_specific_id)
    }

    /// Check if this is a did:web identifier
    pub fn is_web(&self) -> bool {
        self.method == DidMethod::Web
    }

    /// For did:web, extract the domain and path components
    /// Returns (domain, optional_path)
    pub fn web_components(&self) -> Result<(String, Option<Vec<String>>), String> {
        if !self.is_web() {
            return Err("Not a did:web identifier".to_string());
        }

        let parts: Vec<String> = self
            .method_specific_id
            .split(':')
            .map(|s| urlencoding::decode(s).unwrap_or_default().to_string())
            .collect();

        if parts.is_empty() {
            return Err("Invalid did:web identifier".to_string());
        }

        let domain = parts[0].clone();
        let path = if parts.len() > 1 {
            Some(parts[1..].to_vec())
        } else {
            None
        };

        Ok((domain, path))
    }

    /// Construct the HTTPS URL for fetching a did:web document
    /// Following the did:web spec: https://w3c-ccg.github.io/did-method-web/
    pub fn web_document_url(&self) -> Result<String, String> {
        let (domain, path) = self.web_components()?;

        let url = if let Some(path_parts) = path {
            // did:web:example.com:user:alice -> https://example.com/user/alice/did.json
            format!("https://{}/{}/did.json", domain, path_parts.join("/"))
        } else {
            // did:web:example.com -> https://example.com/.well-known/did.json
            format!("https://{}/.well-known/did.json", domain)
        };

        Ok(url)
    }

    /// Validate DID syntax according to W3C spec
    pub fn validate(&self) -> Result<(), String> {
        // Method validation
        if self.method.method_name().is_empty() {
            return Err("DID method cannot be empty".to_string());
        }

        // Method-specific ID validation
        if self.method_specific_id.is_empty() {
            return Err("Method-specific identifier cannot be empty".to_string());
        }

        // Additional validation for did:web
        if self.is_web() {
            self.web_components()?;
        }

        Ok(())
    }
}

impl std::fmt::Display for DidIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::str::FromStr for DidIdentifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_did_web() {
        let did = DidIdentifier::parse("did:web:example.com").unwrap();
        assert_eq!(did.method, DidMethod::Web);
        assert_eq!(did.method_specific_id, "example.com");
        assert_eq!(did.fragment, None);
    }

    #[test]
    fn test_parse_did_web_with_path() {
        let did = DidIdentifier::parse("did:web:example.com:user:alice").unwrap();
        assert_eq!(did.method, DidMethod::Web);
        assert_eq!(did.method_specific_id, "example.com:user:alice");
    }

    #[test]
    fn test_parse_did_with_fragment() {
        let did = DidIdentifier::parse("did:web:example.com#key-1").unwrap();
        assert_eq!(did.method, DidMethod::Web);
        assert_eq!(did.method_specific_id, "example.com");
        assert_eq!(did.fragment, Some("key-1".to_string()));
    }

    #[test]
    fn test_to_string() {
        let did = DidIdentifier {
            method: DidMethod::Web,
            method_specific_id: "example.com".to_string(),
            fragment: None,
        };
        assert_eq!(did.to_string(), "did:web:example.com");
    }

    #[test]
    fn test_to_string_with_fragment() {
        let did = DidIdentifier {
            method: DidMethod::Web,
            method_specific_id: "example.com".to_string(),
            fragment: Some("key-1".to_string()),
        };
        assert_eq!(did.to_string(), "did:web:example.com#key-1");
    }

    #[test]
    fn test_web_document_url_simple() {
        let did = DidIdentifier::parse("did:web:example.com").unwrap();
        assert_eq!(
            did.web_document_url().unwrap(),
            "https://example.com/.well-known/did.json"
        );
    }

    #[test]
    fn test_web_document_url_with_path() {
        let did = DidIdentifier::parse("did:web:example.com:user:alice").unwrap();
        assert_eq!(
            did.web_document_url().unwrap(),
            "https://example.com/user/alice/did.json"
        );
    }

    #[test]
    fn test_web_document_url_with_port() {
        let did = DidIdentifier::parse("did:web:example.com%3A3000").unwrap();
        assert_eq!(
            did.web_document_url().unwrap(),
            "https://example.com:3000/.well-known/did.json"
        );
    }

    #[test]
    fn test_invalid_did_no_prefix() {
        let result = DidIdentifier::parse("web:example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_did_no_method() {
        let result = DidIdentifier::parse("did:example.com");
        assert!(result.is_err());
    }
}
