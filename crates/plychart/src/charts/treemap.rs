#[cfg(target_arch = "wasm32")]
pub fn draw(
    ctx: &web_sys::CanvasRenderingContext2d,
    items: &[(String, f64)],
    area: &crate::types::ChartArea,
) {
    if items.is_empty() {
        return;
    }

    let total: f64 = items.iter().map(|(_, v)| v).sum();
    if total <= 0.0 {
        return;
    }

    let palette = ["#3b82f6", "#ef4444", "#22c55e", "#f59e0b", "#8b5cf6", "#ec4899", "#06b6d4", "#f97316"];
    let gap = 1.0;
    let mut rects: Vec<(f64, f64, f64, f64, usize)> = Vec::new();

    fn slice(
        items: &[(String, f64)],
        total: f64,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        offset: usize,
        rects: &mut Vec<(f64, f64, f64, f64, usize)>,
    ) {
        if items.is_empty() || w <= 0.0 || h <= 0.0 {
            return;
        }
        if items.len() == 1 {
            rects.push((x, y, w, h, offset));
            return;
        }
        let mid = items.len() / 2;
        let left_sum: f64 = items[..mid].iter().map(|(_, v)| v).sum();
        let right_sum: f64 = items[mid..].iter().map(|(_, v)| v).sum();
        let left_total = left_sum + right_sum;

        if w >= h {
            let split = w * (left_sum / left_total);
            slice(items, total, x, y, split - gap, h, offset, rects);
            slice(items, total, x + split, y, w - split, h, offset + mid, rects);
        } else {
            let split = h * (left_sum / left_total);
            slice(items, total, x, y, w, split - gap, offset, rects);
            slice(items, total, x, y + split, w, h - split, offset + mid, rects);
        }
    }

    slice(items, total, area.x, area.y, area.w, area.h, 0, &mut rects);

    for (i, (rx, ry, rw, rh, idx)) in rects.iter().enumerate() {
        let color = palette[i % palette.len()];
        ctx.set_fill_style(&color.into());
        ctx.fill_rect(*rx, *ry, *rw, *rh);

        let (ref name, val) = items[*idx];
        let text = format!("{} {:.2}", name, val);
        ctx.set_fill_style(&"#ffffff".into());
        ctx.set_font("10px sans-serif");
        ctx.fill_text(&text, *rx + 3.0, *ry + 12.0).ok();
    }
}
