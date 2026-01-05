use crate::models::SpaceCommon;
use crate::*;

use aws_config::BehaviorVersion;
use aws_config::{Region, defaults};
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::config::Credentials;
use aws_sdk_s3::primitives::ByteStream;
use uuid::Uuid;

use genpdf::elements;
use genpdf::fonts::{FontData, FontFamily};
use genpdf::{Alignment, Element, Mm, Position, RenderResult, Size, style};
use std::collections::HashMap;
use std::path::PathBuf;

struct VCenterText {
    text: String,
    align: Alignment,
    height_mm: i32,
    pad_v_mm: i32,
    pad_h_mm: i32,
    style: style::Style,
}

impl VCenterText {
    fn new(
        text: impl Into<String>,
        align: Alignment,
        height_mm: i32,
        pad_v_mm: i32,
        pad_h_mm: i32,
        style: style::Style,
    ) -> Self {
        Self {
            text: text.into(),
            align,
            height_mm,
            pad_v_mm,
            pad_h_mm,
            style,
        }
    }
}

impl Element for VCenterText {
    fn render(
        &mut self,
        context: &genpdf::Context,
        mut area: genpdf::render::Area<'_>,
        mut base_style: style::Style,
    ) -> std::result::Result<RenderResult, genpdf::error::Error> {
        base_style.merge(self.style);

        let row_h = Mm::from(self.height_mm);
        area.set_height(row_h);

        let mut inner = area.clone();
        inner.add_margins(genpdf::Margins::trbl(
            self.pad_v_mm,
            self.pad_h_mm,
            self.pad_v_mm,
            self.pad_h_mm,
        ));

        let line_h = base_style.line_height(&context.font_cache);
        let avail_h = inner.size().height;

        let mut y = if avail_h > line_h {
            (avail_h - line_h) / 2.0
        } else {
            Mm::from(0)
        };

        y = y - (line_h * 0.20);
        if y < Mm::from(0) {
            y = Mm::from(0);
        }

        inner.add_offset(Position::new(0, y));

        let mut p = elements::Paragraph::new(self.text.clone()).aligned(self.align);
        let r = p.render(context, inner, base_style)?;

        Ok(RenderResult {
            size: Size::new(area.size().width, row_h),
            has_more: r.has_more,
        })
    }
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
    _space: &SpaceCommon,
    lda: &[TopicRow],
    _network: &[NetworkCentralityRow],
    _tf_idf: &[TfidfRow],
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

    doc.push(elements::Break::new(1));
    doc.push(elements::Paragraph::new("• 토론 분석_LDA").styled(style::Style::new().bold()));
    doc.push(elements::Break::new(1));

    let rows = lda_to_table_rows(lda, 5);

    let header_h: i32 = 14;
    let row_h: i32 = 12;
    let pad_v: i32 = 2;
    let pad_h: i32 = 3;

    let mut table = elements::TableLayout::new(vec![4, 16]);
    table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

    {
        let hs = style::Style::new().bold();
        let mut r = table.row();
        r.push_element(VCenterText::new(
            "주제",
            Alignment::Center,
            header_h,
            pad_v,
            pad_h,
            hs,
        ));
        r.push_element(VCenterText::new(
            "키워드",
            Alignment::Center,
            header_h,
            pad_v,
            pad_h,
            hs,
        ));
        r.push()
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    }

    for (topic, keywords) in rows {
        let mut r = table.row();
        r.push_element(VCenterText::new(
            topic,
            Alignment::Center,
            row_h,
            pad_v,
            pad_h,
            style::Style::new(),
        ));
        r.push_element(VCenterText::new(
            keywords,
            Alignment::Center,
            row_h,
            pad_v,
            pad_h,
            style::Style::new(),
        ));
        r.push()
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    }

    doc.push(table);

    let mut out: Vec<u8> = Vec::new();
    doc.render(&mut out)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    Ok(out)
}

fn lda_to_table_rows(lda: &[TopicRow], top_topics: usize) -> Vec<(String, String)> {
    let mut by_topic: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    for r in lda {
        by_topic
            .entry(r.topic.clone())
            .or_default()
            .push((r.keyword.clone(), r.weight));
    }

    let mut topics: Vec<(usize, String)> = by_topic
        .keys()
        .filter_map(|t| {
            let n = t
                .split('_')
                .last()
                .and_then(|x| x.parse::<usize>().ok())
                .unwrap_or(9999);
            Some((n, t.clone()))
        })
        .collect();

    topics.sort_by(|a, b| a.0.cmp(&b.0));
    topics.truncate(top_topics);

    let mut out = Vec::new();
    for (_, t) in topics {
        let mut kws = by_topic.remove(&t).unwrap_or_default();
        kws.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let label_num = t
            .split('_')
            .last()
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap_or(1);
        let topic_label = format!("토픽 {}", label_num);

        let keywords = kws
            .into_iter()
            .take(10)
            .map(|(k, _)| k)
            .collect::<Vec<_>>()
            .join(", ");

        out.push((topic_label, keywords));
    }
    out
}
