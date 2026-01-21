use crate::*;

use crate::utils::aws::S3Client;
use crate::utils::reports::utils;

pub async fn build_space_html_contents(html_contents: String) -> Result<Vec<u8>> {
    utils::render_report_pdf_bytes(html_contents).await
}

pub async fn upload_report_pdf_to_s3(
    pdf_bytes: Vec<u8>,
    s3: &S3Client,
) -> Result<(String, String)> {
    utils::upload_report_pdf(pdf_bytes, s3).await
}
