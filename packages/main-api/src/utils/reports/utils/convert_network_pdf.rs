use crate::utils::reports::push_html_rendered_block;
use crate::*;
use ab_glyph::Font;
use ab_glyph::{FontArc, PxScale, ScaleFont};
use genpdf::{Alignment, Element, Scale as PdfScale, elements, style};
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use imageproc::drawing::{
    draw_filled_circle_mut, draw_hollow_circle_mut, draw_line_segment_mut, draw_text_mut,
};
use std::collections::HashMap;
use std::io::Cursor;

pub fn convert_network_pdf(
    doc: &mut genpdf::Document,
    network: NetworkGraph,
    network_html_contents: String,
    font: &FontArc,
) -> Result<()> {
    if network.nodes.is_empty() {
        return Ok(());
    }

    doc.push(elements::PageBreak::new());
    doc.push(
        elements::Paragraph::new("• 토론 분석_Text Network").styled(style::Style::new().bold()),
    );
    doc.push(elements::Break::new(0.8));

    let img = render_network_chart(&network, font)?;
    let dpi: f64 = 300.0;

    let mut bytes = Vec::new();
    DynamicImage::ImageRgb8(img.to_rgb8())
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Jpeg)
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?;

    let (scale, _) = compute_image_scale_for_a4(img.width(), img.height(), dpi);

    let el = elements::Image::from_reader(Cursor::new(bytes))
        .map_err(|e| crate::Error::InternalServerError(e.to_string()))?
        .with_alignment(Alignment::Center)
        .with_scale(PdfScale::new(scale, scale));

    doc.push(el);
    doc.push(elements::Break::new(0.6));

    if !network_html_contents.trim().is_empty() {
        push_html_rendered_block(doc, &network_html_contents)?;
    }

    Ok(())
}

fn compute_image_scale_for_a4(w_px: u32, h_px: u32, dpi: f64) -> (f32, u32) {
    let page_w_mm: f64 = 210.0;
    let page_h_mm: f64 = 297.0;
    let margin_mm: f64 = 10.0;

    let content_w_mm = page_w_mm - margin_mm * 2.0;
    let content_h_mm = page_h_mm - margin_mm * 2.0;

    let img_w_mm = (w_px as f64) * 25.4 / dpi;
    let img_h_mm = (h_px as f64) * 25.4 / dpi;

    let scale_w = content_w_mm / img_w_mm;
    let scale_h = content_h_mm / img_h_mm;

    let scale = (scale_w.min(scale_h)) as f32;

    let max_img_h_px = ((content_h_mm / (scale as f64)) * dpi / 25.4)
        .floor()
        .max(300.0) as u32;

    (scale, max_img_h_px)
}

fn render_network_chart(network: &NetworkGraph, font: &FontArc) -> Result<DynamicImage> {
    let w: u32 = 4200;
    let h: u32 = 2100;
    let pad: f64 = 50.0;

    let mut node_idx: HashMap<String, usize> = HashMap::new();
    for (i, n) in network.nodes.iter().enumerate() {
        node_idx.insert(n.node.clone(), i);
    }

    let mut edges: Vec<(usize, usize, u32)> = Vec::new();
    for e in &network.edges {
        if let (Some(&a), Some(&b)) = (node_idx.get(&e.source), node_idx.get(&e.target)) {
            if a != b {
                edges.push((a, b, e.weight.max(1)));
            }
        }
    }

    let n = network.nodes.len().max(1);

    let mut deg_w: Vec<f64> = vec![0.0f64; n];
    for (a, b, wgt) in &edges {
        let ww = *wgt as f64;
        deg_w[*a] += ww;
        deg_w[*b] += ww;
    }

    let (min_deg, max_deg) = deg_w.iter().fold((f64::INFINITY, 0.0f64), |(mn, mx), v| {
        (mn.min(*v), mx.max(*v))
    });
    let denom_deg = (max_deg - min_deg).max(1e-9);

    let mut r_px: Vec<i32> = vec![0; n];
    for i in 0..n {
        let t = ((deg_w[i] - min_deg) / denom_deg).clamp(0.0, 1.0);
        let r = 62.0 + t * 88.0;
        r_px[i] = r.round() as i32;
    }

    let mut pos = fr_layout(n, &edges, 2300, 0.48, 0.0016, 6.2);

    collision_relax(&mut pos, &r_px, 260, 1.30);

    widen_to_aspect(
        &mut pos,
        &r_px,
        (w as f64 - pad * 2.0) / (h as f64 - pad * 2.0),
    );
    fit_to_canvas(&mut pos, &r_px, w as f64, h as f64, pad);

    let white = Rgba([255, 255, 255, 255]);
    let node_fill = Rgba([200, 226, 236, 255]);
    let node_stroke = Rgba([90, 90, 90, 255]);
    let edge_col = Rgba([160, 160, 160, 35]);
    let text_col = Rgba([30, 30, 30, 255]);

    let mut img = RgbaImage::from_pixel(w, h, white);

    for (a, b, wgt) in edges.iter() {
        let (x1, y1) = pos[*a];
        let (x2, y2) = pos[*b];

        let thick = (1.0f64 + ((*wgt as f64).log10().max(0.0) * 1.15)).round() as i32;
        let thick = thick.clamp(1, 3);

        for t in -(thick / 2)..=(thick / 2) {
            draw_line_segment_mut(
                &mut img,
                (x1 as f32, (y1 + t) as f32),
                (x2 as f32, (y2 + t) as f32),
                edge_col,
            );
        }
    }

    for i in 0..n {
        let (x, y) = pos[i];
        let r = r_px[i];

        let border = ((r as f32) * 0.06).round() as i32;
        let border = border.clamp(3, 10);
        let inner_r = (r - border).max(1);

        draw_filled_circle_mut(&mut img, (x, y), r, node_stroke);
        draw_filled_circle_mut(&mut img, (x, y), inner_r, node_fill);
        draw_hollow_circle_mut(&mut img, (x, y), r, node_stroke);

        let label = &network.nodes[i].node;
        let font_px = ((r as f32) * 0.80).clamp(34.0, 72.0);

        let (tw, th) = text_size_px(font, font_px, label);
        let tx = (x - (tw / 2)).clamp(10, (w as i32) - 10);
        let ty = (y - (th / 2)).clamp(10, (h as i32) - 10);

        draw_text_mut(&mut img, text_col, tx, ty, font_px, font, label);
    }

    Ok(DynamicImage::ImageRgba8(img))
}

fn text_size_px(font: &FontArc, px: f32, s: &str) -> (i32, i32) {
    let scaled = font.as_scaled(PxScale::from(px));
    let mut w = 0.0f32;
    for ch in s.chars() {
        let id = scaled.glyph_id(ch);
        w += scaled.h_advance(id);
    }
    let h = (px * 1.05).ceil() as i32;
    (w.ceil() as i32, h)
}

fn fr_layout(
    n: usize,
    edges: &[(usize, usize, u32)],
    iters: usize,
    t0: f64,
    t_min: f64,
    k_boost: f64,
) -> Vec<(i32, i32)> {
    let mut pos: Vec<(f64, f64)> = Vec::with_capacity(n);

    for i in 0..n {
        let a = (i as f64) * (std::f64::consts::TAU / (n as f64));
        let r = 0.49;
        let (jx, jy) = jitter01(i as u64);
        let jx = (jx - 0.5) * 0.07;
        let jy = (jy - 0.5) * 0.07;
        pos.push((0.5 + r * a.cos() + jx, 0.5 + r * a.sin() + jy));
    }

    let area = 1.0f64;
    let k = (area / (n as f64)).sqrt() * k_boost;

    let mut t = t0;

    for step in 0..iters {
        let mut disp: Vec<(f64, f64)> = vec![(0.0f64, 0.0f64); n];

        for v in 0..n {
            for u in (v + 1)..n {
                let dx = pos[v].0 - pos[u].0;
                let dy = pos[v].1 - pos[u].1;
                let dist = (dx * dx + dy * dy).sqrt().max(1e-6);
                let force = (k * k) / dist;

                let fx = (dx / dist) * force;
                let fy = (dy / dist) * force;

                disp[v].0 += fx;
                disp[v].1 += fy;
                disp[u].0 -= fx;
                disp[u].1 -= fy;
            }
        }

        for (a, b, wgt) in edges {
            let dx = pos[*a].0 - pos[*b].0;
            let dy = pos[*a].1 - pos[*b].1;
            let dist = (dx * dx + dy * dy).sqrt().max(1e-6);

            let w = ((*wgt as f64).ln_1p()).powf(0.78).clamp(0.65, 2.65);

            let force = (dist * dist / k) * w;
            let fx = (dx / dist) * force;
            let fy = (dy / dist) * force;

            disp[*a].0 -= fx;
            disp[*a].1 -= fy;
            disp[*b].0 += fx;
            disp[*b].1 += fy;
        }

        for v in 0..n {
            let dx = disp[v].0;
            let dy = disp[v].1;
            let dist = (dx * dx + dy * dy).sqrt().max(1e-9);

            let lim = t.min(dist);
            pos[v].0 += (dx / dist) * lim;
            pos[v].1 += (dy / dist) * lim;
        }

        ellipse_bound(&mut pos, 0.5, 0.5, 0.49, 0.38);

        let cool = 1.0 - ((step as f64) / (iters as f64)).powf(1.18);
        t = (t0 * cool).max(t_min);
    }

    pos.into_iter()
        .map(|(x, y)| ((x * 1000.0) as i32, (y * 1000.0) as i32))
        .collect()
}

fn ellipse_bound(pos: &mut [(f64, f64)], cx: f64, cy: f64, ax: f64, ay: f64) {
    let eps = 1e-12;
    for p in pos.iter_mut() {
        let dx = p.0 - cx;
        let dy = p.1 - cy;
        let u = dx / ax;
        let v = dy / ay;
        let r = (u * u + v * v).sqrt().max(eps);
        if r > 0.995 {
            let s = 0.995 / r;
            p.0 = cx + dx * s;
            p.1 = cy + dy * s;
        }
    }
}

fn jitter01(mut x: u64) -> (f64, f64) {
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    let a = (x & 0xFFFF_FFFF) as u32;

    x = x.wrapping_mul(0x9E37_79B9_7F4A_7C15);
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    let b = (x & 0xFFFF_FFFF) as u32;

    let fa = (a as f64) / (u32::MAX as f64);
    let fb = (b as f64) / (u32::MAX as f64);
    (fa, fb)
}

fn collision_relax(pos: &mut [(i32, i32)], r_px: &[i32], iters: usize, gain: f64) {
    let n = pos.len();
    let mut p: Vec<(f64, f64)> = pos.iter().map(|(x, y)| (*x as f64, *y as f64)).collect();

    for _ in 0..iters {
        let mut moved = false;

        for i in 0..n {
            for j in (i + 1)..n {
                let dx = p[i].0 - p[j].0;
                let dy = p[i].1 - p[j].1;
                let dist = (dx * dx + dy * dy).sqrt().max(1e-6);

                let min_d = (r_px[i] + r_px[j]) as f64 * gain + 10.0;
                if dist < min_d {
                    let push = (min_d - dist) * 0.5;
                    let ux = dx / dist;
                    let uy = dy / dist;

                    p[i].0 += ux * push;
                    p[i].1 += uy * push;
                    p[j].0 -= ux * push;
                    p[j].1 -= uy * push;

                    moved = true;
                }
            }
        }

        if !moved {
            break;
        }
    }

    for i in 0..n {
        pos[i] = (p[i].0.round() as i32, p[i].1.round() as i32);
    }
}

fn widen_to_aspect(pos: &mut [(i32, i32)], r_px: &[i32], target_aspect: f64) {
    let n = pos.len();
    if n == 0 {
        return;
    }

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for i in 0..n {
        let (x, y) = pos[i];
        let r = r_px[i] as f64;
        min_x = min_x.min(x as f64 - r);
        max_x = max_x.max(x as f64 + r);
        min_y = min_y.min(y as f64 - r);
        max_y = max_y.max(y as f64 + r);
    }

    let bw = (max_x - min_x).max(1.0);
    let bh = (max_y - min_y).max(1.0);
    let aspect = bw / bh;

    if aspect >= target_aspect * 0.97 {
        return;
    }

    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;

    let raw = (target_aspect / aspect).max(1.0);
    let fx = raw.powf(0.92).clamp(1.0, 1.55);
    let fy = (1.0 / raw).powf(0.18).clamp(0.86, 1.0);

    for p in pos.iter_mut() {
        let x = p.0 as f64;
        let y = p.1 as f64;
        let nx = cx + (x - cx) * fx;
        let ny = cy + (y - cy) * fy;
        *p = (nx.round() as i32, ny.round() as i32);
    }
}

fn fit_to_canvas(pos: &mut [(i32, i32)], r_px: &[i32], w: f64, h: f64, pad: f64) {
    let n = pos.len();
    if n == 0 {
        return;
    }

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for i in 0..n {
        let (x, y) = pos[i];
        let r = r_px[i] as f64;
        min_x = min_x.min(x as f64 - r);
        max_x = max_x.max(x as f64 + r);
        min_y = min_y.min(y as f64 - r);
        max_y = max_y.max(y as f64 + r);
    }

    let bw = (max_x - min_x).max(1.0);
    let bh = (max_y - min_y).max(1.0);

    let avail_w = (w - pad * 2.0).max(1.0);
    let avail_h = (h - pad * 2.0).max(1.0);

    let s = (avail_w / bw).min(avail_h / bh) * 0.996;

    let cx_src = (min_x + max_x) * 0.5;
    let cy_src = (min_y + max_y) * 0.5;

    let cx_dst = w * 0.5;
    let cy_dst = h * 0.5;

    for i in 0..n {
        let (x, y) = pos[i];
        let nx = ((x as f64 - cx_src) * s + cx_dst).round() as i32;
        let ny = ((y as f64 - cy_src) * s + cy_dst).round() as i32;
        pos[i] = (nx, ny);
    }
}
