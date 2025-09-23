use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use dto::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;

/// StatusList2021 implementation for W3C Verifiable Credentials
/// Provides bitstring-based status tracking with gzip compression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusList2021 {
    /// The compressed bitstring as base64url-encoded data
    #[serde(rename = "encodedList")]
    pub encoded_list: String,
    /// Size of the status list (number of bits)
    pub size: usize,
    /// Optional metadata about the status list
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl StatusList2021 {
    /// Create a new status list with the specified size
    /// All bits are initially set to 0 (valid/not revoked)
    pub fn new(size: usize) -> Result<Self> {
        let mut bitstring = BitString::new(size);
        bitstring.fill(false); // Initialize all to valid/not revoked

        let encoded_list = Self::compress_bitstring(&bitstring)?;

        Ok(Self {
            encoded_list,
            size,
            metadata: None,
        })
    }

    /// Create a status list from an existing encoded list
    pub fn from_encoded(encoded_list: String, size: usize) -> Result<Self> {
        // Validate by decompressing
        let _bitstring = Self::decompress_bitstring(&encoded_list, size)?;

        Ok(Self {
            encoded_list,
            size,
            metadata: None,
        })
    }

    /// Get the status at a specific index
    pub fn get_status(&self, index: usize) -> Result<bool> {
        if index >= self.size {
            return Err(Error::StatusListIndexOutOfBounds {
                index,
                size: self.size,
            });
        }

        let bitstring = Self::decompress_bitstring(&self.encoded_list, self.size)?;
        Ok(bitstring.get(index))
    }

    /// Set the status at a specific index
    pub fn set_status(&mut self, index: usize, status: bool) -> Result<()> {
        if index >= self.size {
            return Err(Error::StatusListIndexOutOfBounds {
                index,
                size: self.size,
            });
        }

        let mut bitstring = Self::decompress_bitstring(&self.encoded_list, self.size)?;
        bitstring.set(index, status);
        self.encoded_list = Self::compress_bitstring(&bitstring)?;

        Ok(())
    }

    /// Revoke a credential at the specified index
    pub fn revoke(&mut self, index: usize) -> Result<()> {
        self.set_status(index, true)
    }

    /// Suspend a credential at the specified index (same as revoke for basic status)
    pub fn suspend(&mut self, index: usize) -> Result<()> {
        self.set_status(index, true)
    }

    /// Restore a credential at the specified index (unrevoke)
    pub fn restore(&mut self, index: usize) -> Result<()> {
        self.set_status(index, false)
    }

    /// Check if a credential is revoked
    pub fn is_revoked(&self, index: usize) -> Result<bool> {
        self.get_status(index)
    }

    /// Check if a credential is valid (not revoked)
    pub fn is_valid(&self, index: usize) -> Result<bool> {
        self.get_status(index).map(|status| !status)
    }

    /// Get all revoked indices
    pub fn get_revoked_indices(&self) -> Result<Vec<usize>> {
        let bitstring = Self::decompress_bitstring(&self.encoded_list, self.size)?;
        let mut revoked = Vec::new();

        for i in 0..self.size {
            if bitstring.get(i) {
                revoked.push(i);
            }
        }

        Ok(revoked)
    }

    /// Get the number of revoked credentials
    pub fn revoked_count(&self) -> Result<usize> {
        let bitstring = Self::decompress_bitstring(&self.encoded_list, self.size)?;
        let mut count = 0;

        for i in 0..self.size {
            if bitstring.get(i) {
                count += 1;
            }
        }

        Ok(count)
    }

    /// Update multiple statuses at once
    pub fn batch_update(&mut self, updates: &[(usize, bool)]) -> Result<()> {
        let mut bitstring = Self::decompress_bitstring(&self.encoded_list, self.size)?;

        for &(index, status) in updates {
            if index >= self.size {
                return Err(Error::StatusListIndexOutOfBounds {
                    index,
                    size: self.size,
                });
            }
            bitstring.set(index, status);
        }

        self.encoded_list = Self::compress_bitstring(&bitstring)?;
        Ok(())
    }

    /// Compress a bitstring using gzip and encode as base64url
    fn compress_bitstring(bitstring: &BitString) -> Result<String> {
        use flate2::{Compression, write::GzEncoder};
        use std::io::Write;

        let bytes = bitstring.to_bytes();
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&bytes)
            .map_err(|e| Error::StatusListCompression(e.to_string()))?;

        let compressed = encoder
            .finish()
            .map_err(|e| Error::StatusListCompression(e.to_string()))?;

        Ok(URL_SAFE_NO_PAD.encode(&compressed))
    }

    /// Decompress a base64url-encoded gzipped bitstring
    fn decompress_bitstring(encoded: &str, size: usize) -> Result<BitString> {
        use flate2::read::GzDecoder;
        use std::io::Read;

        let compressed = URL_SAFE_NO_PAD
            .decode(encoded)
            .map_err(|e| Error::StatusListBase64Decode(e.to_string()))?;
        let mut decoder = GzDecoder::new(&compressed[..]);
        let mut decompressed = Vec::new();

        decoder
            .read_to_end(&mut decompressed)
            .map_err(|e| Error::StatusListDecompression(e.to_string()))?;

        BitString::from_bytes(decompressed, size)
    }

    /// Get statistics about the status list
    pub fn get_stats(&self) -> Result<StatusListStats> {
        let revoked_count = self.revoked_count()?;
        let valid_count = self.size - revoked_count;
        let compression_ratio = self.encoded_list.len() as f64 / (self.size / 8) as f64;

        Ok(StatusListStats {
            total_size: self.size,
            valid_count,
            revoked_count,
            compression_ratio,
            encoded_size: self.encoded_list.len(),
        })
    }
}

/// Statistics about a status list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusListStats {
    pub total_size: usize,
    pub valid_count: usize,
    pub revoked_count: usize,
    pub compression_ratio: f64,
    pub encoded_size: usize,
}

/// Internal bitstring representation
#[derive(Debug, Clone)]
struct BitString {
    bits: Vec<u8>,
    size: usize,
}

impl BitString {
    fn new(size: usize) -> Self {
        let byte_size = (size + 7) / 8; // Round up to nearest byte
        Self {
            bits: vec![0; byte_size],
            size,
        }
    }

    fn from_bytes(bytes: Vec<u8>, size: usize) -> Result<Self> {
        let expected_byte_size = (size + 7) / 8;
        if bytes.len() != expected_byte_size {
            return Err(Error::StatusListDecompression(format!(
                "Invalid byte length: expected {}, got {}",
                expected_byte_size,
                bytes.len()
            )));
        }

        Ok(Self { bits: bytes, size })
    }

    fn get(&self, index: usize) -> bool {
        if index >= self.size {
            return false;
        }

        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= self.bits.len() {
            return false;
        }

        (self.bits[byte_index] & (1 << bit_index)) != 0
    }

    fn set(&mut self, index: usize, value: bool) {
        if index >= self.size {
            return;
        }

        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= self.bits.len() {
            return;
        }

        if value {
            self.bits[byte_index] |= 1 << bit_index;
        } else {
            self.bits[byte_index] &= !(1 << bit_index);
        }
    }

    fn fill(&mut self, value: bool) {
        let fill_byte = if value { 0xFF } else { 0x00 };
        self.bits.fill(fill_byte);

        // Clear any extra bits in the last byte
        if self.size % 8 != 0 {
            let last_byte_index = self.bits.len() - 1;
            let valid_bits = self.size % 8;
            let mask = (1 << valid_bits) - 1;

            if value {
                self.bits[last_byte_index] = mask;
            } else {
                self.bits[last_byte_index] = 0;
            }
        }
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.bits.clone()
    }
}

/// StatusList2021 credential context and type constants
pub mod constants {
    pub const STATUS_LIST_2021_CONTEXT: &str = "https://w3id.org/vc/status-list/2021/v1";
    pub const STATUS_LIST_2021_TYPE: &str = "StatusList2021";
    pub const STATUS_LIST_2021_CREDENTIAL_TYPE: &str = "StatusList2021Credential";
    pub const REVOCATION_PURPOSE: &str = "revocation";
    pub const SUSPENSION_PURPOSE: &str = "suspension";
}

/// Helper functions for creating status list credentials
pub mod credential_helpers {
    use super::*;
    use chrono::Utc;
    use serde_json::{Value, json};

    /// Create a StatusList2021 credential
    pub fn create_status_list_credential(
        id: &str,
        issuer: &str,
        status_list: &StatusList2021,
        purpose: &str,
    ) -> Value {
        json!({
            "@context": [
                "https://www.w3.org/2018/credentials/v1",
                constants::STATUS_LIST_2021_CONTEXT
            ],
            "id": id,
            "type": ["VerifiableCredential", constants::STATUS_LIST_2021_CREDENTIAL_TYPE],
            "issuer": issuer,
            "issuanceDate": Utc::now().to_rfc3339(),
            "credentialSubject": {
                "id": format!("{}#list", id),
                "type": constants::STATUS_LIST_2021_TYPE,
                "statusPurpose": purpose,
                "encodedList": status_list.encoded_list
            }
        })
    }

    /// Create a status entry for a credential
    pub fn create_status_entry(
        status_list_credential_id: &str,
        list_index: usize,
        purpose: &str,
    ) -> Value {
        json!({
            "id": format!("{}#{}", status_list_credential_id, list_index),
            "type": constants::STATUS_LIST_2021_TYPE,
            "statusPurpose": purpose,
            "statusListIndex": list_index.to_string(),
            "statusListCredential": status_list_credential_id
        })
    }
}

/// Status list manager for handling multiple status lists
pub struct StatusListManager {
    lists: HashMap<String, StatusList2021>,
}

impl StatusListManager {
    pub fn new() -> Self {
        Self {
            lists: HashMap::new(),
        }
    }

    /// Create a new status list
    pub fn create_list(&mut self, id: String, size: usize) -> Result<()> {
        let status_list = StatusList2021::new(size)?;
        self.lists.insert(id, status_list);
        Ok(())
    }

    /// Get a status list by ID
    pub fn get_list(&self, id: &str) -> Option<&StatusList2021> {
        self.lists.get(id)
    }

    /// Get a mutable status list by ID
    pub fn get_list_mut(&mut self, id: &str) -> Option<&mut StatusList2021> {
        self.lists.get_mut(id)
    }

    /// Update status in a specific list
    pub fn update_status(&mut self, list_id: &str, index: usize, status: bool) -> Result<()> {
        let list = self.lists.get_mut(list_id).ok_or_else(|| {
            Error::StatusListCompression(format!("Status list not found: {}", list_id))
        })?;

        list.set_status(index, status)
    }

    /// Check status in a specific list
    pub fn check_status(&self, list_id: &str, index: usize) -> Result<bool> {
        let list = self.lists.get(list_id).ok_or_else(|| {
            Error::StatusListCompression(format!("Status list not found: {}", list_id))
        })?;

        list.get_status(index)
    }

    /// Get all list IDs
    pub fn list_ids(&self) -> Vec<&String> {
        self.lists.keys().collect()
    }
}

impl Default for StatusListManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_list_creation() {
        let status_list = StatusList2021::new(1000).unwrap();
        assert_eq!(status_list.size, 1000);
        assert!(!status_list.encoded_list.is_empty());
    }

    #[test]
    fn test_status_operations() {
        let mut status_list = StatusList2021::new(100).unwrap();

        // Initially all should be valid (false = not revoked)
        assert!(!status_list.is_revoked(0).unwrap());
        assert!(status_list.is_valid(0).unwrap());

        // Revoke a credential
        status_list.revoke(5).unwrap();
        assert!(status_list.is_revoked(5).unwrap());
        assert!(!status_list.is_valid(5).unwrap());

        // Restore it
        status_list.restore(5).unwrap();
        assert!(!status_list.is_revoked(5).unwrap());
        assert!(status_list.is_valid(5).unwrap());
    }

    #[test]
    fn test_batch_updates() {
        let mut status_list = StatusList2021::new(100).unwrap();

        let updates = vec![
            (10, true),  // revoke
            (20, true),  // revoke
            (30, false), // restore (was already false)
        ];

        status_list.batch_update(&updates).unwrap();

        assert!(status_list.is_revoked(10).unwrap());
        assert!(status_list.is_revoked(20).unwrap());
        assert!(!status_list.is_revoked(30).unwrap());

        assert_eq!(status_list.revoked_count().unwrap(), 2);
    }

    #[test]
    fn test_compression_decompression() {
        let mut status_list = StatusList2021::new(1000).unwrap();

        // Revoke some credentials
        status_list.revoke(100).unwrap();
        status_list.revoke(200).unwrap();
        status_list.revoke(300).unwrap();

        // Create new status list from encoded data
        let status_list2 =
            StatusList2021::from_encoded(status_list.encoded_list.clone(), status_list.size)
                .unwrap();

        // Should have same revocation status
        assert!(status_list2.is_revoked(100).unwrap());
        assert!(status_list2.is_revoked(200).unwrap());
        assert!(status_list2.is_revoked(300).unwrap());
        assert!(!status_list2.is_revoked(400).unwrap());
    }

    #[test]
    fn test_out_of_bounds() {
        let status_list = StatusList2021::new(100).unwrap();

        let result = status_list.get_status(100);
        assert!(matches!(
            result,
            Err(Error::StatusListIndexOutOfBounds { .. })
        ));
    }

    #[test]
    fn test_status_list_manager() {
        let mut manager = StatusListManager::new();

        manager.create_list("revocation".to_string(), 1000).unwrap();
        manager.create_list("suspension".to_string(), 1000).unwrap();

        // Update statuses
        manager.update_status("revocation", 100, true).unwrap();
        manager.update_status("suspension", 200, true).unwrap();

        // Check statuses
        assert!(manager.check_status("revocation", 100).unwrap());
        assert!(!manager.check_status("revocation", 200).unwrap());
        assert!(manager.check_status("suspension", 200).unwrap());
        assert!(!manager.check_status("suspension", 100).unwrap());

        assert_eq!(manager.list_ids().len(), 2);
    }

    #[test]
    fn test_credential_helpers() {
        let status_list = StatusList2021::new(1000).unwrap();

        let credential = credential_helpers::create_status_list_credential(
            "https://example.com/status/1",
            "https://example.com",
            &status_list,
            constants::REVOCATION_PURPOSE,
        );

        assert_eq!(
            credential["type"][1],
            constants::STATUS_LIST_2021_CREDENTIAL_TYPE
        );
        assert_eq!(
            credential["credentialSubject"]["statusPurpose"],
            constants::REVOCATION_PURPOSE
        );

        let status_entry = credential_helpers::create_status_entry(
            "https://example.com/status/1",
            100,
            constants::REVOCATION_PURPOSE,
        );

        assert_eq!(status_entry["statusListIndex"], "100");
        assert_eq!(status_entry["statusPurpose"], constants::REVOCATION_PURPOSE);
    }
}
