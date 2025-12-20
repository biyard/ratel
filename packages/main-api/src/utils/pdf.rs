use pdf_extract::extract_text_from_mem;

/// Extract text from PDF bytes for a specific page range
/// 
/// # Arguments
/// * `pdf_bytes` - The raw PDF file content
/// * `current_page` - The page the user is currently viewing (1-indexed)
/// * `_context_pages` - Number of pages before and after to include for context (currently unused)
/// 
/// # Returns
/// Extracted text from the specified page range, or an error if extraction fails
pub fn extract_pdf_text(
    pdf_bytes: &[u8],
    current_page: Option<u32>,
    _context_pages: u32,
) -> Result<String, String> {
    // Extract all text from PDF
    let full_text = extract_text_from_mem(pdf_bytes)
        .map_err(|e| format!("Failed to extract PDF text: {}", e))?;

    // For now, return the full text
    // TODO: Implement page-specific extraction if needed
    // The pdf-extract crate doesn't provide easy page-by-page access,
    // so we return the full document text with a note about the current page
    
    if let Some(page_num) = current_page {
        Ok(format!(
            "[User is viewing page {} of the PDF]\n\n{}",
            page_num, full_text
        ))
    } else {
        Ok(full_text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pdf_text_empty() {
        let result = extract_pdf_text(&[], None, 2);
        assert!(result.is_err());
    }
}
