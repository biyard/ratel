use crate::models::SpaceCommon;
use crate::utils::reports::convert_lda_pdf;
use crate::utils::reports::convert_tfidf_pdf;
use crate::*;

use aws_config::BehaviorVersion;
use aws_config::{Region, defaults};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;

use genpdf::fonts::{FontData, FontFamily};
use genpdf::{Alignment, Element, elements, style};

use std::path::PathBuf;

fn resolve_asset_dir(path: &str) -> PathBuf {
    let p = PathBuf::from(path);
    if p.is_absolute() {
        return p;
    }

    std::env::current_exe()
        .ok()
        .and_then(|exe| exe.parent().map(|d| d.join(&p)))
        .unwrap_or_else(|| p)
}

pub fn build_space_report_pdf(
    lda: &[TopicRow],
    _network: &[NetworkCentralityRow],
    tf_idf: &[TfidfRow],
) -> Result<Vec<u8>> {
    let cfg = crate::config::get();

    let base = resolve_asset_dir(cfg.report_fonts_dir);
    let p = |name: &str| -> PathBuf { base.join(name) };

    let regular = FontData::load(p("NotoSansKR-Regular.ttf"), None)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    let bold = FontData::load(p("NotoSansKR-Bold.ttf"), None).unwrap_or_else(|_| regular.clone());
    let italic =
        FontData::load(p("NotoSansKR-Italic.ttf"), None).unwrap_or_else(|_| regular.clone());
    let bold_italic =
        FontData::load(p("NotoSansKR-BoldItalic.ttf"), None).unwrap_or_else(|_| bold.clone());

    let font_family = FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    };

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

    convert_lda_pdf(&mut doc, lda)?;
    doc.push(elements::Break::new(1.0));

    convert_tfidf_pdf(&mut doc, tf_idf, &p("NotoSansKR-Regular.ttf"))?;

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
    let key = format!("{}/reports/{}.pdf", asset_dir, id);

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
