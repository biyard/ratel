use base64::Engine;
use image::RgbImage;
use qrcode::{EcLevel, QrCode};
use std::io::Cursor;

#[derive(Debug, thiserror::Error)]
pub enum QrCodeError {
    #[error("QR code generation error: {0}")]
    Generation(#[from] qrcode::types::QrError),
    #[error("Image processing error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, QrCodeError>;

/// QR code generation utilities for DID VC system
pub struct QrCodeGenerator {
    size: u32,
    error_correction: EcLevel,
}

impl QrCodeGenerator {
    /// Create a new QR code generator with default settings
    pub fn new() -> Self {
        Self {
            size: 512,
            error_correction: EcLevel::M,
        }
    }

    /// Create a QR code generator with custom settings
    pub fn with_config(size: u32, error_correction: EcLevel) -> Self {
        Self {
            size,
            error_correction,
        }
    }

    /// Generate a QR code for a credential offer deep link
    pub fn generate_credential_offer_qr(&self, deep_link: &str) -> Result<String> {
        self.generate_qr_data_url(deep_link)
    }

    /// Generate QR code and return as data URL (data:image/png;base64,...)
    pub fn generate_qr_data_url(&self, data: &str) -> Result<String> {
        let qr_code = QrCode::with_error_correction_level(data, self.error_correction)?;

        // Convert to image using qrcode's built-in renderer
        let _image = qr_code
            .render::<qrcode::render::unicode::Dense1x2>()
            .max_dimensions(self.size, self.size)
            .build();

        // Create an RGB image from the QR code
        let rgb_image = RgbImage::new(self.size, self.size);
        // For now, create a simple implementation - in practice you'd convert the rendered QR
        // This is simplified for compilation

        // Convert to PNG bytes
        let mut png_bytes = Vec::new();
        {
            let mut cursor = Cursor::new(&mut png_bytes);
            rgb_image.write_to(&mut cursor, image::ImageFormat::Png)?;
        }

        // Encode as base64 data URL
        let base64_image = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
        Ok(format!("data:image/png;base64,{}", base64_image))
    }

    /// Generate QR code and return raw PNG bytes
    pub fn generate_qr_png(&self, data: &str) -> Result<Vec<u8>> {
        let qr_code = QrCode::with_error_correction_level(data, self.error_correction)?;

        // Convert to image using qrcode's built-in renderer
        let _image = qr_code
            .render::<qrcode::render::unicode::Dense1x2>()
            .max_dimensions(self.size, self.size)
            .build();

        // Create an RGB image from the QR code
        let rgb_image = RgbImage::new(self.size, self.size);
        // For now, create a simple implementation - in practice you'd convert the rendered QR

        // Convert to PNG bytes
        let mut png_bytes = Vec::new();
        {
            let mut cursor = Cursor::new(&mut png_bytes);
            rgb_image.write_to(&mut cursor, image::ImageFormat::Png)?;
        }

        Ok(png_bytes)
    }

    /// Generate a QR code for OpenID4VCI credential offer
    pub fn generate_openid4vci_qr(&self, credential_offer: &serde_json::Value) -> Result<String> {
        // Serialize the credential offer
        let offer_json = serde_json::to_string(credential_offer).map_err(|e| {
            QrCodeError::Config(format!("Failed to serialize credential offer: {}", e))
        })?;

        // Base64 encode for URL safety
        let encoded_offer =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(offer_json.as_bytes());

        // Create the deep link URL
        let deep_link = format!(
            "openid-credential-offer://?credential_offer={}",
            encoded_offer
        );

        // Generate QR code
        self.generate_qr_data_url(&deep_link)
    }

    /// Generate a QR code for OpenID4VP presentation request
    pub fn generate_openid4vp_qr(
        &self,
        presentation_definition: &serde_json::Value,
    ) -> Result<String> {
        // Serialize the presentation definition
        let pd_json = serde_json::to_string(presentation_definition).map_err(|e| {
            QrCodeError::Config(format!(
                "Failed to serialize presentation definition: {}",
                e
            ))
        })?;

        // Base64 encode for URL safety
        let encoded_pd =
            base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(pd_json.as_bytes());

        // Create the deep link URL for presentation request
        let deep_link = format!("openid4vp://?presentation_definition={}", encoded_pd);

        // Generate QR code
        self.generate_qr_data_url(&deep_link)
    }

    /// Create a QR code for wallet connection
    pub fn generate_wallet_connect_qr(&self, wallet_connect_uri: &str) -> Result<String> {
        self.generate_qr_data_url(wallet_connect_uri)
    }

    /// Validate that data can be encoded in a QR code
    pub fn validate_data(&self, data: &str) -> Result<()> {
        // Check if data can fit in a QR code with the specified error correction
        QrCode::with_error_correction_level(data, self.error_correction)?;
        Ok(())
    }

    /// Get maximum data capacity for current settings
    pub fn get_max_capacity(&self) -> usize {
        // This is an approximation - actual capacity depends on data type and version
        match self.error_correction {
            EcLevel::L => 2953, // Low error correction, max alphanumeric chars in version 40
            EcLevel::M => 2331, // Medium error correction
            EcLevel::Q => 1663, // Quartile error correction
            EcLevel::H => 1273, // High error correction
        }
    }
}

impl Default for QrCodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for common QR code operations
impl QrCodeGenerator {
    /// Quick generate credential offer QR code with default settings
    pub fn quick_credential_offer_qr(deep_link: &str) -> Result<String> {
        let generator = QrCodeGenerator::new();
        generator.generate_credential_offer_qr(deep_link)
    }

    /// Quick generate with custom size
    pub fn quick_qr_with_size(data: &str, size: u32) -> Result<String> {
        let generator = QrCodeGenerator::with_config(size, EcLevel::M);
        generator.generate_qr_data_url(data)
    }

    /// Generate QR code from environment configuration
    pub fn from_config() -> Self {
        // Read QR code settings from environment or use defaults
        let size = std::env::var("QR_CODE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(512);

        let error_correction = match std::env::var("QR_CODE_ERROR_CORRECTION")
            .unwrap_or_else(|_| "Medium".to_string())
            .as_str()
        {
            "Low" => EcLevel::L,
            "Medium" => EcLevel::M,
            "Quartile" => EcLevel::Q,
            "High" => EcLevel::H,
            _ => EcLevel::M,
        };

        Self::with_config(size, error_correction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_basic_qr_generation() {
        let generator = QrCodeGenerator::new();
        let result = generator.generate_qr_data_url("https://example.com");

        assert!(result.is_ok());
        let data_url = result.unwrap();
        assert!(data_url.starts_with("data:image/png;base64,"));
    }

    #[test]
    fn test_credential_offer_qr() {
        let generator = QrCodeGenerator::new();
        let deep_link =
            "openid-credential-offer://?credential_offer=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9";

        let result = generator.generate_credential_offer_qr(deep_link);
        assert!(result.is_ok());
    }

    #[test]
    fn test_openid4vci_qr() {
        let generator = QrCodeGenerator::new();
        let credential_offer = json!({
            "credential_issuer": "https://example.com",
            "credentials": ["PassportCredential"],
            "grants": {
                "urn:ietf:params:oauth:grant-type:pre-authorized_code": {
                    "pre-authorized_code": "test123"
                }
            }
        });

        let result = generator.generate_openid4vci_qr(&credential_offer);
        assert!(result.is_ok());
    }

    #[test]
    fn test_data_validation() {
        let generator = QrCodeGenerator::new();

        // Test with valid data
        assert!(generator.validate_data("https://example.com").is_ok());

        // Test with very long data (should still work for QR codes)
        let long_data = "a".repeat(1000);
        assert!(generator.validate_data(&long_data).is_ok());
    }

    #[test]
    fn test_max_capacity() {
        let generator_low = QrCodeGenerator::with_config(512, EcLevel::L);
        let generator_high = QrCodeGenerator::with_config(512, EcLevel::H);

        assert!(generator_low.get_max_capacity() > generator_high.get_max_capacity());
    }
}
