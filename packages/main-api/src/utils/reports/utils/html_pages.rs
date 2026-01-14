use crate::*;

use genpdf::Element;
use genpdf::{Alignment, Mm, Position, RenderResult, Size, elements, style};
use std::collections::VecDeque;

pub fn push_html_rendered_block(doc: &mut genpdf::Document, html_fragment: &str) -> Result<()> {
    let html_fragment = html_fragment.trim();
    if html_fragment.is_empty() {
        return Ok(());
    }

    let blocks = html_to_blocks(html_fragment);

    doc.push(elements::Break::new(1.0));

    let total = blocks.len();
    for (idx, b) in blocks.into_iter().enumerate() {
        let is_last = idx + 1 == total;

        match b {
            Block::Heading { level, text } => {
                let t = text.trim();
                if t.is_empty() {
                    continue;
                }

                let st = style::Style::new().bold().with_font_size(match level {
                    1 => 18,
                    2 => 16,
                    _ => 14,
                });

                doc.push(
                    elements::Paragraph::new(t.to_string())
                        .aligned(Alignment::Left)
                        .styled(st),
                );
                if !is_last {
                    doc.push(elements::Break::new(0.6));
                }
            }

            Block::Paragraph { text, bold, italic } => {
                let t = text.trim();
                if t.is_empty() {
                    continue;
                }

                let mut st = style::Style::new();
                if bold {
                    st = st.bold();
                }
                if italic {
                    st = st.italic();
                }

                doc.push(
                    elements::Paragraph::new(t.to_string())
                        .aligned(Alignment::Left)
                        .styled(st),
                );
                if !is_last {
                    doc.push(elements::Break::new(0.35));
                }
            }

            Block::ListItem {
                level,
                text,
                bold,
                italic,
            } => {
                let t = text.trim();
                if t.is_empty() {
                    continue;
                }

                let indent = "  ".repeat(level.saturating_sub(1) * 2);
                let line = format!("{indent}• {t}");

                let mut st = style::Style::new();
                if bold {
                    st = st.bold();
                }
                if italic {
                    st = st.italic();
                }

                doc.push(
                    elements::Paragraph::new(line)
                        .aligned(Alignment::Left)
                        .styled(st),
                );
                if !is_last {
                    doc.push(elements::Break::new(0.15));
                }
            }

            Block::Table { rows } => {
                if rows.is_empty() {
                    continue;
                }
                let cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
                if cols == 0 {
                    continue;
                }

                let mut table = elements::TableLayout::new(vec![1; cols]);
                table.set_cell_decorator(elements::FrameCellDecorator::new(true, true, false));

                let pad_v: i32 = 2;
                let pad_h: i32 = 3;

                for row in rows {
                    let mut r = table.row();
                    for ci in 0..cols {
                        let cell = row.get(ci).cloned().unwrap_or_default();

                        let mut st = style::Style::new();
                        if cell.header || cell.bold {
                            st = st.bold();
                        }
                        if cell.italic {
                            st = st.italic();
                        }

                        let align = if cell.header {
                            Alignment::Center
                        } else {
                            Alignment::Left
                        };
                        r.push_element(PCell::new(cell.text, align, pad_v, pad_h, st));
                    }
                    r.push()
                        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;
                }

                doc.push(elements::Break::new(0.35));
                doc.push(table);
                doc.push(elements::Break::new(0.5));
            }

            Block::BlankLine => {
                if !is_last {
                    doc.push(elements::Break::new(0.5));
                }
            }
        }
    }

    Ok(())
}

#[derive(Clone, Default)]
struct TableCell {
    text: String,
    header: bool,
    bold: bool,
    italic: bool,
}

#[derive(Clone)]
enum Block {
    Heading {
        level: u8,
        text: String,
    },
    Paragraph {
        text: String,
        bold: bool,
        italic: bool,
    },
    ListItem {
        level: usize,
        text: String,
        bold: bool,
        italic: bool,
    },
    Table {
        rows: Vec<Vec<TableCell>>,
    },
    BlankLine,
}

#[derive(Default)]
struct ParseState {
    buf: String,

    bold_depth: usize,
    italic_depth: usize,

    list_level: usize,
    in_li: bool,

    heading_level: Option<u8>,

    in_table: bool,
    table_rows: Vec<Vec<TableCell>>,
    cur_row: Vec<TableCell>,
    in_cell: bool,
    cell_is_header: bool,
    cell_bold_seen: bool,
    cell_italic_seen: bool,
}

fn html_to_blocks(html: &str) -> Vec<Block> {
    let mut out: Vec<Block> = Vec::new();
    let mut st = ParseState::default();

    let mut i = 0usize;
    let bytes = html.as_bytes();

    while i < bytes.len() {
        if bytes[i] == b'<' {
            if let Some((tag, next_i)) = parse_tag(html, i) {
                handle_tag(&mut out, &mut st, tag);
                i = next_i;
                continue;
            } else {
                st.buf.push('<');
                i += 1;
                continue;
            }
        }

        if bytes[i] == b'&' {
            if let Some((decoded, next_i)) = parse_entity(html, i) {
                st.buf.push_str(&decoded);
                i = next_i;
                continue;
            }
        }

        let (ch, next_i) = next_char(html, i);
        st.buf.push_str(ch);
        i = next_i;
    }

    flush_current(&mut out, &mut st, FlushReason::Eof);
    finalize_table_if_needed(&mut out, &mut st);

    out
}

#[derive(Debug)]
struct Tag {
    name: String,
    is_end: bool,
    is_self_closing: bool,
    _raw_attrs: String,
}

fn parse_tag(s: &str, start: usize) -> Option<(Tag, usize)> {
    let bytes = s.as_bytes();
    if bytes.get(start)? != &b'<' {
        return None;
    }

    let mut j = start + 1;
    while j < bytes.len() && bytes[j] != b'>' {
        j += 1;
    }
    if j >= bytes.len() {
        return None;
    }

    let inside = s[start + 1..j].trim();
    let next_i = j + 1;

    if inside.starts_with("!--") {
        return Some((
            Tag {
                name: "!comment".to_string(),
                is_end: false,
                is_self_closing: true,
                _raw_attrs: String::new(),
            },
            next_i,
        ));
    }

    let is_end = inside.starts_with('/');
    let inside = inside.strip_prefix('/').unwrap_or(inside).trim();

    let is_self_closing = inside.ends_with('/');
    let inside = inside.strip_suffix('/').unwrap_or(inside).trim();

    let mut parts = inside.splitn(2, char::is_whitespace);
    let name = parts.next()?.trim().to_lowercase();
    let attrs = parts.next().unwrap_or("").trim().to_string();

    Some((
        Tag {
            name,
            is_end,
            is_self_closing,
            _raw_attrs: attrs,
        },
        next_i,
    ))
}

fn handle_tag(out: &mut Vec<Block>, st: &mut ParseState, tag: Tag) {
    if tag.name == "!comment" {
        return;
    }

    let name = tag.name.as_str();

    if !tag.is_end && (tag.is_self_closing || name == "br" || name == "hr") {
        match name {
            "br" => st.buf.push('\n'),
            "hr" => {
                flush_current(out, st, FlushReason::BlockBoundary);
                if st.in_table {
                    push_cell_text(st, "");
                } else {
                    out.push(Block::BlankLine);
                }
            }
            _ => {}
        }
        return;
    }

    if !tag.is_end {
        match name {
            "table" => {
                flush_current(out, st, FlushReason::BlockBoundary);
                finalize_table_if_needed(out, st);
                st.in_table = true;
                st.table_rows.clear();
                st.cur_row.clear();
                st.in_cell = false;
            }
            "tr" => {
                if st.in_table {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    finalize_cell_if_needed(st);
                    finalize_row_if_needed(st);
                    st.cur_row = Vec::new();
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                }
            }
            "th" | "td" => {
                if st.in_table {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    finalize_cell_if_needed(st);
                    st.in_cell = true;
                    st.cell_is_header = name == "th";
                    st.cell_bold_seen = false;
                    st.cell_italic_seen = false;
                    st.buf.clear();
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                }
            }

            "p" | "div" | "section" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                }
            }
            "h1" | "h2" | "h3" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    st.heading_level = Some(match name {
                        "h1" => 1,
                        "h2" => 2,
                        _ => 3,
                    });
                }
            }
            "ul" | "ol" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    st.list_level += 1;
                }
            }
            "li" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    st.in_li = true;
                }
            }
            "strong" | "b" => {
                st.bold_depth += 1;
                if st.in_table && st.in_cell {
                    st.cell_bold_seen = true;
                }
            }
            "em" | "i" => {
                st.italic_depth += 1;
                if st.in_table && st.in_cell {
                    st.cell_italic_seen = true;
                }
            }
            _ => {}
        }
    } else {
        match name {
            "table" => {
                flush_current(out, st, FlushReason::BlockBoundary);
                finalize_cell_if_needed(st);
                finalize_row_if_needed(st);
                finalize_table_if_needed(out, st);
            }
            "tr" => {
                if st.in_table {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    finalize_cell_if_needed(st);
                    finalize_row_if_needed(st);
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                }
            }
            "th" | "td" => {
                if st.in_table {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    finalize_cell_if_needed(st);
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                }
            }

            "p" | "div" | "section" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                }
            }
            "h1" | "h2" | "h3" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::HeadingEnd);
                    st.heading_level = None;
                }
            }
            "li" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::LiEnd);
                    st.in_li = false;
                }
            }
            "ul" | "ol" => {
                if st.in_table && st.in_cell {
                    st.buf.push('\n');
                } else {
                    flush_current(out, st, FlushReason::BlockBoundary);
                    st.list_level = st.list_level.saturating_sub(1);
                }
            }
            "strong" | "b" => st.bold_depth = st.bold_depth.saturating_sub(1),
            "em" | "i" => st.italic_depth = st.italic_depth.saturating_sub(1),
            _ => {}
        }
    }
}

#[derive(Clone, Copy)]
enum FlushReason {
    BlockBoundary,
    LiEnd,
    HeadingEnd,
    Eof,
}

fn flush_current(out: &mut Vec<Block>, st: &mut ParseState, reason: FlushReason) {
    let raw = st.buf.clone();
    st.buf.clear();

    let text = normalize_text(&raw);
    if text.is_empty() {
        return;
    }

    let bold = st.bold_depth > 0;
    let italic = st.italic_depth > 0;

    if st.in_table && st.in_cell {
        push_cell_text(st, &text);
        return;
    }

    if let Some(level) = st.heading_level {
        out.push(Block::Heading { level, text });
        return;
    }

    if st.in_li {
        out.push(Block::ListItem {
            level: st.list_level.max(1),
            text,
            bold,
            italic,
        });
        return;
    }

    match reason {
        FlushReason::Eof
        | FlushReason::BlockBoundary
        | FlushReason::HeadingEnd
        | FlushReason::LiEnd => {
            out.push(Block::Paragraph { text, bold, italic });
        }
    }
}

fn push_cell_text(st: &mut ParseState, text: &str) {
    if st.buf.is_empty() {
        st.buf.push_str(text);
    } else {
        if !st.buf.ends_with('\n') {
            st.buf.push('\n');
        }
        st.buf.push_str(text);
    }
}

fn finalize_cell_if_needed(st: &mut ParseState) {
    if !st.in_table || !st.in_cell {
        return;
    }

    let text = normalize_text(&st.buf);
    st.buf.clear();

    let mut cell = TableCell::default();
    cell.text = text;
    cell.header = st.cell_is_header;
    cell.bold = st.cell_bold_seen;
    cell.italic = st.cell_italic_seen;

    st.cur_row.push(cell);

    st.in_cell = false;
    st.cell_is_header = false;
    st.cell_bold_seen = false;
    st.cell_italic_seen = false;
}

fn finalize_row_if_needed(st: &mut ParseState) {
    if !st.in_table {
        return;
    }
    if st.cur_row.is_empty() {
        return;
    }
    st.table_rows.push(std::mem::take(&mut st.cur_row));
}

fn finalize_table_if_needed(out: &mut Vec<Block>, st: &mut ParseState) {
    if !st.in_table {
        return;
    }

    if !st.table_rows.is_empty() {
        out.push(Block::Table {
            rows: std::mem::take(&mut st.table_rows),
        });
    } else {
        st.table_rows.clear();
    }

    st.in_table = false;
    st.in_cell = false;
    st.cur_row.clear();
    st.heading_level = None;
    st.in_li = false;
    st.list_level = 0;
}

fn normalize_text(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_space = false;

    for ch in s.chars() {
        let ch = match ch {
            '\u{00A0}' => ' ',
            _ => ch,
        };

        if ch == '\n' {
            if !out.ends_with('\n') {
                out.push('\n');
            }
            prev_space = false;
            continue;
        }

        if ch.is_whitespace() {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(ch);
            prev_space = false;
        }
    }

    out.trim().to_string()
}

fn next_char(s: &str, i: usize) -> (&str, usize) {
    let rest = &s[i..];
    let mut it = rest.char_indices();
    let _ = it.next().unwrap();
    let next = it.next().map(|(idx, _)| idx).unwrap_or(rest.len());
    (&rest[..next], i + next)
}

fn parse_entity(s: &str, start: usize) -> Option<(String, usize)> {
    let bytes = s.as_bytes();
    if bytes.get(start)? != &b'&' {
        return None;
    }

    let mut j = start + 1;
    let max = (start + 32).min(bytes.len());
    while j < max && bytes[j] != b';' && bytes[j] != b' ' && bytes[j] != b'<' && bytes[j] != b'\n' {
        j += 1;
    }
    if j >= bytes.len() || bytes[j] != b';' {
        return None;
    }

    let ent = &s[start + 1..j];
    let next_i = j + 1;

    let decoded = if let Some(num) = ent.strip_prefix("#x").or_else(|| ent.strip_prefix("#X")) {
        u32::from_str_radix(num, 16)
            .ok()
            .and_then(std::char::from_u32)
            .map(|c| c.to_string())
    } else if let Some(num) = ent.strip_prefix('#') {
        num.parse::<u32>()
            .ok()
            .and_then(std::char::from_u32)
            .map(|c| c.to_string())
    } else {
        match ent {
            "amp" => Some("&".to_string()),
            "lt" => Some("<".to_string()),
            "gt" => Some(">".to_string()),
            "quot" => Some("\"".to_string()),
            "apos" => Some("'".to_string()),
            "nbsp" => Some(" ".to_string()),
            _ => None,
        }
    };

    decoded.map(|d| (d, next_i))
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
