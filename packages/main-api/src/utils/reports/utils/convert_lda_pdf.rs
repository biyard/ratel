use crate::*;

use crate::utils::reports::push_html_rendered_block;
use genpdf::{Alignment, Element, Mm, Position, RenderResult, Size, elements, style};
use std::collections::{HashMap, HashSet};

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

struct PCell {
    text: String,
    align: Alignment,
    pad_v_mm: i32,
    pad_h_mm: i32,
    style: style::Style,
}

impl PCell {
    fn new(
        text: impl Into<String>,
        align: Alignment,
        pad_v_mm: i32,
        pad_h_mm: i32,
        style: style::Style,
    ) -> Self {
        Self {
            text: text.into(),
            align,
            pad_v_mm,
            pad_h_mm,
            style,
        }
    }
}

impl Element for PCell {
    fn render(
        &mut self,
        context: &genpdf::Context,
        area: genpdf::render::Area<'_>,
        mut base_style: style::Style,
    ) -> std::result::Result<RenderResult, genpdf::error::Error> {
        base_style.merge(self.style);

        let mut inner = area.clone();
        inner.add_margins(genpdf::Margins::trbl(
            self.pad_v_mm,
            self.pad_h_mm,
            self.pad_v_mm,
            self.pad_h_mm,
        ));

        let mut p = elements::Paragraph::new(self.text.clone()).aligned(self.align);
        let r = p.render(context, inner, base_style)?;

        Ok(RenderResult {
            size: Size::new(
                area.size().width,
                r.size.height + Mm::from(self.pad_v_mm * 2),
            ),
            has_more: r.has_more,
        })
    }
}

pub fn convert_lda_pdf(
    doc: &mut genpdf::Document,
    lda: &[TopicRow],
    lda_html_contents: String,
) -> Result<()> {
    let topic_cnt = lda
        .iter()
        .map(|r| r.topic.as_str())
        .collect::<HashSet<_>>()
        .len()
        .max(1);

    doc.push(elements::Paragraph::new("• 토론 분석_LDA").styled(style::Style::new().bold()));
    doc.push(elements::Break::new(1.0));

    let rows = lda_to_table_rows(lda, topic_cnt);

    let header_h: i32 = 14;
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
        r.push_element(PCell::new(
            topic,
            Alignment::Center,
            pad_v,
            pad_h,
            style::Style::new(),
        ));
        r.push_element(PCell::new(
            keywords,
            Alignment::Center,
            pad_v,
            pad_h,
            style::Style::new(),
        ));
        r.push()
            .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
    }

    doc.push(table);

    if !lda_html_contents.trim().is_empty() {
        push_html_rendered_block(doc, &lda_html_contents)?;
    }

    Ok(())
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
    topics.truncate(top_topics.max(1));

    let mut out = Vec::new();
    for (_, t) in topics {
        let mut kws = by_topic.remove(&t).unwrap_or_default();
        kws.sort_by(|a, b| b.cmp(a));

        let topic_label = t.clone();
        let keywords = kws.into_iter().take(10).collect::<Vec<_>>().join(", ");
        out.push((topic_label, keywords));
    }
    out
}
