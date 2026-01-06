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

use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use imageproc::drawing::{
    draw_filled_rect_mut, draw_hollow_rect_mut, draw_line_segment_mut, draw_text_mut,
};
use imageproc::rect::Rect;

use std::io::Cursor;
use std::path::{Path as StdPath, PathBuf};

use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use genpdf::Scale as PdfScale;

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

fn load_ttf_font(path: &StdPath) -> Result<FontArc> {
    let bytes =
        std::fs::read(path).map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    FontArc::try_from_vec(bytes)
        .map_err(|e| crate::Error::InternalServerError(format!("failed to load ttf font: {e}")))
}

fn nice_step(max_v: f64, ticks: i32) -> f64 {
    let raw = (max_v / (ticks as f64)).max(1e-9);
    let pow = 10_f64.powf(raw.log10().floor());
    let frac = raw / pow;
    let nice = if frac <= 1.0 {
        1.0
    } else if frac <= 2.0 {
        2.0
    } else if frac <= 5.0 {
        5.0
    } else {
        10.0
    };
    nice * pow
}

fn text_w_px(font: &FontArc, scale: f32, s: &str) -> i32 {
    let scaled = font.as_scaled(PxScale::from(scale));
    let mut w = 0.0f32;
    for ch in s.chars() {
        let id = scaled.glyph_id(ch);
        w += scaled.h_advance(id);
    }
    w.ceil() as i32
}

pub fn render_tfidf_chart_image(
    tf_idf: &[TfidfRow],
    top_n: usize,
    font_path: &std::path::Path,
    title: &str,
) -> Result<DynamicImage> {
    let mut data = tf_idf.to_vec();
    data.sort_by(|a, b| {
        b.tf_idf
            .partial_cmp(&a.tf_idf)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    data.truncate(top_n);
    if data.is_empty() {
        return Ok(DynamicImage::ImageRgba8(RgbaImage::new(1, 1)));
    }

    let font = load_ttf_font(font_path)?;
    let n = data.len() as i32;

    let w: u32 = 2400;

    let label_scale: f32 = 62.0;
    let value_scale: f32 = 60.0;

    let inner_l = 40i32;
    let inner_r = 64i32;
    let inner_t = 34i32;
    let inner_b = 42i32;

    let header_h = 128i32;

    let label_gap = 28i32;
    let max_label_w = data
        .iter()
        .map(|r| text_w_px(&font, label_scale, &r.keyword))
        .max()
        .unwrap_or(0);

    let left_label_w = (max_label_w + label_gap + 12).max(120);
    let right_pad = 120i32;

    let bar_h = 64i32;
    let gap = 18i32;
    let axis_area = 112i32;

    let h: u32 = (inner_t + header_h + n * bar_h + (n - 1) * gap + axis_area + inner_b) as u32;

    let white = Rgba([255, 255, 255, 255]);
    let border = Rgba([120, 120, 120, 255]);
    let bar = Rgba([32, 84, 115, 255]);
    let text = Rgba([60, 60, 60, 255]);

    let mut img = RgbaImage::from_pixel(w, h, white);

    let outer = Rect::at(0, 0).of_size(w - 1, h - 1);
    draw_hollow_rect_mut(&mut img, outer, border);
    let outer2 = Rect::at(1, 1).of_size(w - 3, h - 3);
    draw_hollow_rect_mut(&mut img, outer2, border);

    let chart_x0 = inner_l + left_label_w;
    let chart_x1 = (w as i32) - inner_r - right_pad;

    let chart_y0 = inner_t + header_h;
    let chart_h = n * bar_h + (n - 1) * gap;
    let chart_y1 = chart_y0 + chart_h;

    let title_scale: f32 = 86.0;
    let title_w = text_w_px(&font, title_scale, title);
    let title_x = ((w as i32) / 2 - title_w / 2).max(inner_l);
    let title_y = inner_t + 6;

    draw_text_mut(&mut img, text, title_x, title_y, title_scale, &font, title);
    draw_text_mut(
        &mut img,
        text,
        title_x + 1,
        title_y,
        title_scale,
        &font,
        title,
    );
    draw_text_mut(
        &mut img,
        text,
        title_x,
        title_y + 1,
        title_scale,
        &font,
        title,
    );
    draw_text_mut(
        &mut img,
        text,
        title_x + 1,
        title_y + 1,
        title_scale,
        &font,
        title,
    );

    let max_v = data
        .iter()
        .map(|r| r.tf_idf)
        .fold(0.0_f64, |m, v| if v > m { v } else { m })
        .max(0.0001);

    let tick_count = 6;
    let step = nice_step(max_v, tick_count - 1);
    let max_tick = ((max_v / step).ceil() * step).max(step);

    let axis_y = chart_y1 + 24;
    let x_tick_scale: f32 = 56.0;

    for i in 0..tick_count {
        let v = (i as f64) * step;
        if v > max_tick + 1e-9 {
            continue;
        }

        let t = (v / max_tick).min(1.0);
        let x = chart_x0 + (t * ((chart_x1 - chart_x0) as f64)).round() as i32;

        let label = if step >= 1.0 {
            format!("{:.0}", v)
        } else {
            format!("{:.1}", v)
        };

        let lw = text_w_px(&font, x_tick_scale, &label);
        draw_text_mut(
            &mut img,
            text,
            x - (lw / 2),
            axis_y + 16,
            x_tick_scale,
            &font,
            &label,
        );
    }

    let label_right = chart_x0 - label_gap;

    for (i, r) in data.iter().enumerate() {
        let y = chart_y0 + i as i32 * (bar_h + gap);

        let lw = text_w_px(&font, label_scale, &r.keyword);
        let label_x = (label_right - lw).max(inner_l);

        draw_text_mut(
            &mut img,
            text,
            label_x,
            y + 6,
            label_scale,
            &font,
            &r.keyword,
        );

        let bar_w = ((r.tf_idf / max_tick) * ((chart_x1 - chart_x0) as f64)).round() as i32;
        let bar_w = bar_w.max(1);

        let bar_rect = Rect::at(chart_x0, y).of_size(bar_w as u32, bar_h as u32);
        draw_filled_rect_mut(&mut img, bar_rect, bar);

        let val_str = format!("{:.2}", r.tf_idf);
        let val_x = (chart_x0 + bar_w + 16).min((w as i32) - inner_r - 140);
        draw_text_mut(&mut img, text, val_x, y + 6, value_scale, &font, &val_str);
    }

    Ok(DynamicImage::ImageRgba8(img))
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

    doc.push(elements::Break::new(2));
    doc.push(elements::Paragraph::new("• 토론 분석_TF-IDF").styled(style::Style::new().bold()));
    doc.push(elements::Break::new(1));

    let chart = render_tfidf_chart_image(tf_idf, 10, &p("NotoSansKR-Regular.ttf"), "TF-IDF")?;
    let chart_rgb = DynamicImage::ImageRgb8(chart.to_rgb8());

    let mut bytes = Vec::new();
    chart_rgb
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Jpeg)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let page_w_mm: f64 = 210.0;
    let margin_mm: f64 = 10.0;
    let content_w_mm = page_w_mm - margin_mm * 2.0;

    let dpi: f64 = 300.0;
    let img_w_mm = (chart_rgb.width() as f64) * 25.4 / dpi;

    let s = ((content_w_mm / img_w_mm) * 1.03) as f32;

    let chart_el = elements::Image::from_reader(Cursor::new(bytes))
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?
        .with_alignment(Alignment::Center)
        .with_scale(PdfScale::new(s, s));

    doc.push(chart_el);

    let mut out: Vec<u8> = Vec::new();
    doc.render(&mut out)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    Ok(out)
}

fn lda_to_table_rows(lda: &[TopicRow], top_topics: usize) -> Vec<(String, String)> {
    let mut by_topic: HashMap<String, Vec<String>> = HashMap::new();
    for r in lda {
        by_topic
            .entry(r.topic.clone())
            .or_default()
            .push(r.keyword.clone());
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
        kws.sort_by(|a, b| b.cmp(a));

        let label_num = t
            .split('_')
            .last()
            .and_then(|x| x.parse::<usize>().ok())
            .unwrap_or(1);
        let topic_label = format!("토픽 {}", label_num);

        let keywords = kws
            .into_iter()
            .take(10)
            .map(|k| k)
            .collect::<Vec<_>>()
            .join(", ");

        out.push((topic_label, keywords));
    }
    out
}
