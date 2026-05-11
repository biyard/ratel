use super::{Block, BlockKind, ContentBody, ContentDocument, InlineNode, RichText};

impl ContentBody {
    pub fn to_plain_text(&self) -> String {
        match self {
            ContentBody::HtmlContent(html) => strip_html_tags(html),
            ContentBody::StructuredContent(doc) => doc.to_plain_text(),
        }
    }

    pub fn to_html(&self) -> String {
        match self {
            ContentBody::HtmlContent(html) => html.clone(),
            ContentBody::StructuredContent(doc) => doc.to_html(),
        }
    }

    pub fn char_count(&self) -> usize {
        self.to_plain_text().chars().count()
    }
}

impl ContentDocument {
    pub fn to_plain_text(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.append_plain_text(&mut out);
        }
        normalize_whitespace(&out)
    }

    /// Lossy projection used until structured rendering lands in the UI.
    /// Each block becomes one HTML element; inline runs become spans/strong/em.
    pub fn to_html(&self) -> String {
        let mut out = String::new();
        for block in &self.blocks {
            block.append_html(&mut out);
        }
        out
    }
}

impl Block {
    fn append_plain_text(&self, out: &mut String) {
        match &self.kind {
            BlockKind::Paragraph(t)
            | BlockKind::Quote(t)
            | BlockKind::BulletedListItem(t)
            | BlockKind::NumberedListItem(t)
            | BlockKind::Toggle(t) => {
                t.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Heading(h) => {
                h.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Todo(t) => {
                t.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Callout(c) => {
                c.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Code(c) => {
                c.rich_text.append_plain(out);
                out.push('\n');
            }
            BlockKind::Equation(e) => out.push_str(&e.expression),
            BlockKind::Bookmark(b) => {
                b.caption.append_plain(out);
                if !b.url.is_empty() {
                    out.push(' ');
                    out.push_str(&b.url);
                }
            }
            BlockKind::Embed(e) => out.push_str(&e.url),
            BlockKind::Image(m) | BlockKind::Video(m) | BlockKind::File(m) => {
                if let Some(alt) = &m.alt {
                    out.push_str(alt);
                }
                m.caption.append_plain(out);
            }
            BlockKind::Divider | BlockKind::Custom(_) | BlockKind::Unknown => {}
        }
        for child in &self.children {
            child.append_plain_text(out);
        }
    }

    fn append_html(&self, out: &mut String) {
        match &self.kind {
            BlockKind::Paragraph(t) => {
                out.push_str("<p>");
                t.rich_text.append_html(out);
                out.push_str("</p>");
            }
            BlockKind::Heading(h) => {
                let tag = match h.level {
                    super::HeadingLevel::H1 => "h1",
                    super::HeadingLevel::H2 => "h2",
                    super::HeadingLevel::H3 => "h3",
                };
                out.push('<');
                out.push_str(tag);
                out.push('>');
                h.rich_text.append_html(out);
                out.push_str("</");
                out.push_str(tag);
                out.push('>');
            }
            BlockKind::BulletedListItem(t) => {
                out.push_str("<li>");
                t.rich_text.append_html(out);
                out.push_str("</li>");
            }
            BlockKind::NumberedListItem(t) => {
                out.push_str("<li>");
                t.rich_text.append_html(out);
                out.push_str("</li>");
            }
            BlockKind::Quote(t) => {
                out.push_str("<blockquote>");
                t.rich_text.append_html(out);
                out.push_str("</blockquote>");
            }
            BlockKind::Toggle(t) => {
                out.push_str("<details><summary>");
                t.rich_text.append_html(out);
                out.push_str("</summary></details>");
            }
            BlockKind::Todo(t) => {
                let cb = if t.checked { "checked " } else { "" };
                out.push_str("<label><input type=\"checkbox\" disabled ");
                out.push_str(cb);
                out.push('>');
                t.rich_text.append_html(out);
                out.push_str("</label>");
            }
            BlockKind::Code(c) => {
                out.push_str("<pre><code>");
                c.rich_text.append_plain(out);
                out.push_str("</code></pre>");
            }
            BlockKind::Callout(c) => {
                out.push_str("<aside>");
                c.rich_text.append_html(out);
                out.push_str("</aside>");
            }
            BlockKind::Equation(e) => {
                out.push_str("<span class=\"math\">");
                html_escape(&e.expression, out);
                out.push_str("</span>");
            }
            BlockKind::Image(m) => {
                let url = match &m.source {
                    super::MediaSource::External { url } => url.clone(),
                    super::MediaSource::Asset { asset_id } => asset_id.clone(),
                };
                out.push_str("<img src=\"");
                html_escape_attr(&url, out);
                out.push_str("\"");
                if let Some(alt) = &m.alt {
                    out.push_str(" alt=\"");
                    html_escape_attr(alt, out);
                    out.push('"');
                }
                out.push_str(" />");
            }
            BlockKind::Video(m) | BlockKind::File(m) => {
                let url = match &m.source {
                    super::MediaSource::External { url } => url.clone(),
                    super::MediaSource::Asset { asset_id } => asset_id.clone(),
                };
                out.push_str("<a href=\"");
                html_escape_attr(&url, out);
                out.push_str("\">");
                m.caption.append_html(out);
                out.push_str("</a>");
            }
            BlockKind::Bookmark(b) => {
                out.push_str("<a href=\"");
                html_escape_attr(&b.url, out);
                out.push_str("\">");
                b.caption.append_html(out);
                out.push_str("</a>");
            }
            BlockKind::Embed(e) => {
                out.push_str("<a href=\"");
                html_escape_attr(&e.url, out);
                out.push_str("\">");
                html_escape(&e.url, out);
                out.push_str("</a>");
            }
            BlockKind::Divider => out.push_str("<hr />"),
            BlockKind::Custom(_) | BlockKind::Unknown => {}
        }
        for child in &self.children {
            child.append_html(out);
        }
    }
}

impl RichText {
    fn append_plain(&self, out: &mut String) {
        for node in &self.0 {
            match node {
                InlineNode::Text(t) => out.push_str(&t.content),
                InlineNode::Mention(_) => {}
                InlineNode::Equation(e) => out.push_str(&e.expression),
            }
        }
    }

    fn append_html(&self, out: &mut String) {
        for node in &self.0 {
            match node {
                InlineNode::Text(t) => {
                    let mut buf = String::new();
                    html_escape(&t.content, &mut buf);
                    let mut s = buf;
                    if t.annotations.code {
                        s = format!("<code>{s}</code>");
                    }
                    if t.annotations.bold {
                        s = format!("<strong>{s}</strong>");
                    }
                    if t.annotations.italic {
                        s = format!("<em>{s}</em>");
                    }
                    if t.annotations.strikethrough {
                        s = format!("<s>{s}</s>");
                    }
                    if t.annotations.underline {
                        s = format!("<u>{s}</u>");
                    }
                    if let Some(link) = &t.link {
                        let mut href = String::new();
                        html_escape_attr(link, &mut href);
                        s = format!("<a href=\"{href}\">{s}</a>");
                    }
                    out.push_str(&s);
                }
                InlineNode::Mention(_) => {}
                InlineNode::Equation(e) => {
                    out.push_str("<span class=\"math\">");
                    html_escape(&e.expression, out);
                    out.push_str("</span>");
                }
            }
        }
    }
}

fn strip_html_tags(html: &str) -> String {
    let re_img = regex::Regex::new(r"<img[^>]*>").unwrap();
    let no_img = re_img.replace_all(html, "");
    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap();
    let no_tags = re_tags.replace_all(&no_img, "");
    let re_urls = regex::Regex::new(r"https?://[^\s]+").unwrap();
    let no_urls = re_urls.replace_all(&no_tags, "");
    normalize_whitespace(&no_urls)
}

fn normalize_whitespace(s: &str) -> String {
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(s, " ").trim().to_string()
}

fn html_escape(input: &str, out: &mut String) {
    for ch in input.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            _ => out.push(ch),
        }
    }
}

fn html_escape_attr(input: &str, out: &mut String) {
    for ch in input.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(ch),
        }
    }
}
