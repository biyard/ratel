use crate::models::SpaceCommon;
use crate::utils::reports::convert_lda_pdf;
use crate::utils::reports::convert_network_pdf;
use crate::utils::reports::convert_tfidf_pdf;
use crate::*;

use aws_config::BehaviorVersion;
use aws_config::{Region, defaults};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;

use ab_glyph::FontArc;
use genpdf::fonts::{FontData, FontFamily};
use genpdf::{Alignment, Element, elements, style};

use std::path::PathBuf;

async fn download_font_to_temp(
    dir: &tempfile::TempDir,
    client: &reqwest::Client,
    url: &str,
    name: &str,
) -> Result<PathBuf> {
    let bytes = client
        .get(url)
        .send()
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?
        .bytes()
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let path = dir.path().join(name);
    tokio::fs::write(&path, &bytes)
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    Ok(path)
}

pub async fn build_space_report_pdf(
    lda: &[TopicRow],
    lda_html_contents: String,
    network: NetworkGraph,
    network_html_contents: String,
    tf_idf: &[TfidfRow],
    tf_idf_html_contents: String,
) -> Result<Vec<u8>> {
    let metadata = "https://metadata.ratel.foundation/fonts";
    let tmp = tempfile::tempdir().map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    let http = reqwest::Client::new();

    let regular_path = download_font_to_temp(
        &tmp,
        &http,
        &format!("{}/NotoSansKR-Regular.ttf", metadata),
        "NotoSansKR-Regular.ttf",
    )
    .await?;

    let bold_path = download_font_to_temp(
        &tmp,
        &http,
        &format!("{}/NotoSansKR-Bold.ttf", metadata),
        "NotoSansKR-Bold.ttf",
    )
    .await?;

    let italic_path = download_font_to_temp(
        &tmp,
        &http,
        &format!("{}/NotoSansKR-Italic.ttf", metadata),
        "NotoSansKR-Italic.ttf",
    )
    .await?;

    let bold_italic_path = download_font_to_temp(
        &tmp,
        &http,
        &format!("{}/NotoSansKR-BoldItalic.ttf", metadata),
        "NotoSansKR-BoldItalic.ttf",
    )
    .await?;

    let regular = FontData::load(regular_path.clone(), None)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    let bold = FontData::load(bold_path, None).unwrap_or_else(|_| regular.clone());
    let italic = FontData::load(italic_path, None).unwrap_or_else(|_| regular.clone());
    let bold_italic = FontData::load(bold_italic_path, None).unwrap_or_else(|_| bold.clone());

    let font_family = FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    };

    let chart_font_bytes = tokio::fs::read(&regular_path)
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    let chart_font = FontArc::try_from_vec(chart_font_bytes).map_err(|e| {
        crate::Error::InternalServerError(format!("failed to load chart font: {e}"))
    })?;

    let mut doc = genpdf::Document::new(font_family);
    doc.set_title("Space Report");

    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    doc.push(
        elements::Paragraph::new("토론 내용 분석 (정성분석)")
            .aligned(Alignment::Center)
            .styled(style::Style::new().bold()),
    );
    doc.push(elements::Break::new(1.0));

    convert_lda_pdf(&mut doc, lda, lda_html_contents)?;
    doc.push(elements::Break::new(1.0));

    convert_tfidf_pdf(&mut doc, tf_idf, tf_idf_html_contents, &chart_font)?;
    convert_network_pdf(&mut doc, network, network_html_contents, &chart_font)?;

    let mut out: Vec<u8> = Vec::new();
    doc.render(&mut out)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    Ok(out)
}

pub async fn upload_report_pdf_to_s3(pdf_bytes: Vec<u8>) -> Result<(String, String)> {
    let ratel_config = crate::config::get();
    let aws_config = &ratel_config.aws;

    let asset_dir = ratel_config.s3.asset_dir;
    let bucket_name = ratel_config.s3.name;
    let bucket_region = ratel_config.s3.region;

    let env = ratel_config.env;

    let cfg = defaults(BehaviorVersion::latest())
        .region(Region::new(bucket_region))
        .credentials_provider(Credentials::new(
            aws_config.access_key_id,
            aws_config.secret_access_key,
            None,
            None,
            "ratel",
        ))
        .load()
        .await;

    let client = S3Client::new(&cfg);

    let id = Uuid::new_v4();
    let key = format!("{}/{}/reports/{}.pdf", asset_dir, env.to_lowercase(), id);

    client
        .put_object()
        .bucket(bucket_name)
        .key(&key)
        .content_type("application/pdf")
        .body(ByteStream::from(pdf_bytes))
        .send()
        .await
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let uri = ratel_config.s3.get_url(&key);
    Ok((key, uri))
}
