use crate::utils::reports::push_html_rendered_block;
use crate::*;

use genpdf::{
    Alignment, Element, Mm, Position, RenderResult, Scale as PdfScale, Size, elements, style,
};

use ab_glyph::{Font, FontArc, PxScale, ScaleFont};
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use std::io::Cursor;

struct KeepTogetherTfidfBlock {
    title: String,
    img_bytes: Vec<u8>,
    img_h_px: u32,
    dpi: f64,
    scale: f32,
    gap_before_mm: f64,
    gap_after_title_mm: f64,
    rendered: bool,
}

impl KeepTogetherTfidfBlock {
    fn new(
        title: impl Into<String>,
        img_bytes: Vec<u8>,
        img_h_px: u32,
        dpi: f64,
        scale: f32,
        gap_before_mm: f64,
        gap_after_title_mm: f64,
    ) -> Self {
        Self {
            title: title.into(),
            img_bytes,
            img_h_px,
            dpi,
            scale,
            gap_before_mm,
            gap_after_title_mm,
            rendered: false,
        }
    }
}

impl Element for KeepTogetherTfidfBlock {
    fn render(
        &mut self,
        context: &genpdf::Context,
        area: genpdf::render::Area<'_>,
        base_style: style::Style,
    ) -> std::result::Result<RenderResult, genpdf::error::Error> {
        if self.rendered {
            return Ok(RenderResult {
                size: Size::new(Mm::from(0), Mm::from(0)),
                has_more: false,
            });
        }

        let area_h: f64 = area.size().height.into();
        let lh: f64 = base_style.line_height(&context.font_cache).into();

        let img_h_mm_at =
            |sc: f32| -> f64 { (self.img_h_px as f64) * 25.4 / self.dpi * (sc as f64) };

        let mut eff_scale = self.scale;
        let mut img_h_mm = img_h_mm_at(eff_scale);

        let available_for_img = area_h - self.gap_before_mm - lh - self.gap_after_title_mm - 1.0;

        if available_for_img > 0.0 && img_h_mm > available_for_img {
            let factor = (available_for_img / img_h_mm).clamp(0.85, 1.0);
            eff_scale = (eff_scale as f64 * factor) as f32;
            img_h_mm = img_h_mm_at(eff_scale);
        }

        let need = self.gap_before_mm + lh + self.gap_after_title_mm + img_h_mm + 1.0;

        if area_h < need {
            return Ok(RenderResult {
                size: Size::new(Mm::from(0), Mm::from(0)),
                has_more: true,
            });
        }

        let mut y_off = Mm::from(0);

        if self.gap_before_mm > 0.0 {
            let mut a = area.clone();
            a.add_offset(Position::new(0, y_off));
            let mut b = elements::Break::new(self.gap_before_mm);
            let rr = b.render(context, a, base_style)?;
            y_off += rr.size.height;
        }

        {
            let mut a = area.clone();
            a.add_offset(Position::new(0, y_off));
            let mut p =
                elements::Paragraph::new(self.title.clone()).styled(style::Style::new().bold());
            let rr = p.render(context, a, base_style)?;
            y_off += rr.size.height;
        }

        if self.gap_after_title_mm > 0.0 {
            let mut a = area.clone();
            a.add_offset(Position::new(0, y_off));
            let mut b = elements::Break::new(self.gap_after_title_mm);
            let rr = b.render(context, a, base_style)?;
            y_off += rr.size.height;
        }

        {
            let mut a = area.clone();
            a.add_offset(Position::new(0, y_off));

            let mut img_el = elements::Image::from_reader(Cursor::new(self.img_bytes.clone()))?
                .with_alignment(Alignment::Center)
                .with_scale(PdfScale::new(eff_scale, eff_scale));

            let rr = img_el.render(context, a, base_style)?;
            y_off += rr.size.height;
        }

        self.rendered = true;

        Ok(RenderResult {
            size: Size::new(area.size().width, y_off),
            has_more: false,
        })
    }
}

pub fn convert_tfidf_pdf(
    doc: &mut genpdf::Document,
    tf_idf: &[TfidfRow],
    tf_idf_html_contents: String,
    font: &FontArc,
) -> Result<()> {
    let (scale, max_chart_h_px, dpi) = compute_tfidf_scale_and_max_h();

    let mut tfidf_pages =
        render_tfidf_chart_pages(tf_idf, tf_idf.len().max(1), font, "TF-IDF", max_chart_h_px)?;

    if tfidf_pages.is_empty() {
        return Ok(());
    }

    let first = tfidf_pages.remove(0);

    let first_rgb = DynamicImage::ImageRgb8(first.to_rgb8());
    let mut first_bytes = Vec::new();
    first_rgb
        .write_to(&mut Cursor::new(&mut first_bytes), ImageFormat::Jpeg)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let block = KeepTogetherTfidfBlock::new(
        "• 토론 분석_TF-IDF",
        first_bytes,
        first_rgb.height(),
        dpi,
        scale,
        1.0,
        1.0,
    );
    doc.push(block);

    if !tfidf_pages.is_empty() {
        push_tfidf_remaining_pages(doc, tfidf_pages, scale)?;
    }

    if !tf_idf_html_contents.trim().is_empty() {
        push_html_rendered_block(doc, &tf_idf_html_contents)?;
    }

    Ok(())
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

fn compute_tfidf_scale_and_max_h() -> (f32, u32, f64) {
    let page_w_mm: f64 = 210.0;
    let page_h_mm: f64 = 297.0;
    let margin_mm: f64 = 10.0;

    let content_w_mm = page_w_mm - margin_mm * 2.0;
    let content_h_mm = page_h_mm - margin_mm * 2.0;

    let dpi: f64 = 300.0;
    let chart_w_px: f64 = 2400.0;
    let img_w_mm = chart_w_px * 25.4 / dpi;

    let mut scale = (content_w_mm / img_w_mm) as f32;
    scale *= 0.98;

    let max_img_h_px = ((content_h_mm / (scale as f64)) * dpi / 25.4)
        .floor()
        .max(300.0) as u32;

    let reserved_title_mm: f64 = 24.0;
    let reserved_title_px = ((reserved_title_mm / (scale as f64)) * dpi / 25.4)
        .floor()
        .max(0.0) as u32;

    let max_chart_h_px = max_img_h_px.saturating_sub(reserved_title_px).max(600);

    (scale, max_chart_h_px, dpi)
}

pub fn render_tfidf_chart_pages(
    tf_idf: &[TfidfRow],
    top_n: usize,
    font: &FontArc,
    title: &str,
    max_page_h_px: u32,
) -> Result<Vec<DynamicImage>> {
    let mut data = tf_idf.to_vec();
    data.sort_by(|a, b| {
        b.tf_idf
            .partial_cmp(&a.tf_idf)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    data.truncate(top_n.max(1));
    if data.is_empty() {
        return Ok(vec![DynamicImage::ImageRgba8(RgbaImage::new(1, 1))]);
    }

    let w: u32 = 2400;

    let label_scale: f32 = 62.0;
    let value_scale: f32 = 60.0;

    let inner_l = 40i32;
    let inner_r = 64i32;
    let inner_t = 34i32;
    let inner_b = 42i32;

    let header_h = 128i32;
    let label_gap = 28i32;

    let bar_h = 64i32;
    let gap = 18i32;
    let axis_area = 112i32;

    let max_label_w = data
        .iter()
        .map(|r| text_w_px(font, label_scale, &r.keyword))
        .max()
        .unwrap_or(0);

    let left_label_w = (max_label_w + label_gap + 12).max(120);
    let right_pad = 120i32;

    let chart_x0 = inner_l + left_label_w;
    let chart_x1 = (w as i32) - inner_r - right_pad;

    let max_v = data
        .iter()
        .map(|r| r.tf_idf)
        .fold(0.0_f64, |m, v| if v > m { v } else { m })
        .max(0.0001);

    let tick_count = 6;
    let step = nice_step(max_v, tick_count - 1);
    let max_tick = ((max_v / step).ceil() * step).max(step);

    let fixed_top = inner_t + header_h + axis_area + inner_b;
    let available = (max_page_h_px as i32) - fixed_top;
    let bars_per_page = if available <= bar_h {
        1usize
    } else {
        ((available + gap) / (bar_h + gap)).max(1) as usize
    };

    let white = Rgba([255, 255, 255, 255]);
    let border = Rgba([120, 120, 120, 255]);
    let bar = Rgba([32, 84, 115, 255]);
    let text = Rgba([60, 60, 60, 255]);

    let title_scale: f32 = 86.0;
    let x_tick_scale: f32 = 56.0;

    let mut pages: Vec<DynamicImage> = Vec::new();

    let mut start = 0usize;
    while start < data.len() {
        let end = (start + bars_per_page).min(data.len());
        let slice = &data[start..end];
        let n = slice.len() as i32;

        let h: u32 = (inner_t + header_h + n * bar_h + (n - 1) * gap + axis_area + inner_b) as u32;

        let mut img = RgbaImage::from_pixel(w, h, white);

        let outer = Rect::at(0, 0).of_size(w - 1, h - 1);
        draw_hollow_rect_mut(&mut img, outer, border);
        let outer2 = Rect::at(1, 1).of_size(w - 3, h - 3);
        draw_hollow_rect_mut(&mut img, outer2, border);

        let title_w = text_w_px(font, title_scale, title);
        let title_x = ((w as i32) / 2 - title_w / 2).max(inner_l);
        let title_y = inner_t + 6;

        draw_text_mut(&mut img, text, title_x, title_y, title_scale, font, title);
        draw_text_mut(
            &mut img,
            text,
            title_x + 1,
            title_y,
            title_scale,
            font,
            title,
        );
        draw_text_mut(
            &mut img,
            text,
            title_x,
            title_y + 1,
            title_scale,
            font,
            title,
        );
        draw_text_mut(
            &mut img,
            text,
            title_x + 1,
            title_y + 1,
            title_scale,
            font,
            title,
        );

        let chart_y0 = inner_t + header_h;
        let chart_h = n * bar_h + (n - 1) * gap;
        let chart_y1 = chart_y0 + chart_h;

        let axis_y = chart_y1 + 24;

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

            let lw = text_w_px(font, x_tick_scale, &label);
            draw_text_mut(
                &mut img,
                text,
                x - (lw / 2),
                axis_y + 16,
                x_tick_scale,
                font,
                &label,
            );
        }

        let label_right = chart_x0 - label_gap;

        for (i, r) in slice.iter().enumerate() {
            let y = chart_y0 + i as i32 * (bar_h + gap);

            let lw = text_w_px(font, label_scale, &r.keyword);
            let label_x = (label_right - lw).max(inner_l);

            draw_text_mut(
                &mut img,
                text,
                label_x,
                y + 6,
                label_scale,
                font,
                &r.keyword,
            );

            let bar_w = ((r.tf_idf / max_tick) * ((chart_x1 - chart_x0) as f64)).round() as i32;
            let bar_w = bar_w.max(1);

            let bar_rect = Rect::at(chart_x0, y).of_size(bar_w as u32, bar_h as u32);
            draw_filled_rect_mut(&mut img, bar_rect, bar);

            let val_str = format!("{:.2}", r.tf_idf);
            let val_x = (chart_x0 + bar_w + 16).min((w as i32) - inner_r - 140);
            draw_text_mut(&mut img, text, val_x, y + 6, value_scale, font, &val_str);
        }

        pages.push(DynamicImage::ImageRgba8(img));
        start = end;
    }

    Ok(pages)
}

fn push_tfidf_remaining_pages(
    doc: &mut genpdf::Document,
    pages: Vec<DynamicImage>,
    scale: f32,
) -> Result<()> {
    for img in pages {
        doc.push(elements::PageBreak::new());
        doc.push(elements::Paragraph::new("• 토론 분석_TF-IDF").styled(style::Style::new().bold()));
        doc.push(elements::Break::new(1.0));

        let rgb = DynamicImage::ImageRgb8(img.to_rgb8());
        let mut bytes = Vec::new();
        rgb.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Jpeg)
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

        let el = elements::Image::from_reader(Cursor::new(bytes))
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?
            .with_alignment(Alignment::Center)
            .with_scale(PdfScale::new(scale, scale));

        doc.push(el);
    }
    Ok(())
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
