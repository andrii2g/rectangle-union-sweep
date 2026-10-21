use crate::{Rectangle, Result, geometry::length, union_area_with_trace};
use std::fmt::Write;

/// Static, standalone panels show the state after each sampled X event group.
/// Limits bound diagram size; use the area command for larger datasets.
pub fn render_svg(rectangles: &[Rectangle], frames: usize) -> Result<String> {
    if rectangles.len() > 200 {
        return Err("SVG rendering is limited to 200 rectangles".into());
    }
    if !(1..=24).contains(&frames) {
        return Err("SVG frames must be between 1 and 24".into());
    }
    let (metrics, snapshots) = union_area_with_trace(rectangles, frames)?;
    let nonempty: Vec<_> = rectangles.iter().filter(|r| !r.is_empty()).collect();
    let min_x = nonempty.iter().map(|r| r.x1).min().unwrap_or(0);
    let max_x = nonempty.iter().map(|r| r.x2).max().unwrap_or(1);
    let min_y = nonempty.iter().map(|r| r.y1).min().unwrap_or(0);
    let max_y = nonempty.iter().map(|r| r.y2).max().unwrap_or(1);
    // Subtract in integer space before converting, preserving small differences
    // even when coordinates are close to i64::MIN/MAX.
    let px = |x| 34.0 + length(min_x, x) as f64 / length(min_x, max_x) as f64 * 380.0;
    let py = |y| 260.0 - length(min_y, y) as f64 / length(min_y, max_y) as f64 * 175.0;
    let rows = snapshots.len().max(1).div_ceil(2);
    let height = 125 + rows * 370;
    let mut svg = format!(
        r##"<svg xmlns="http://www.w3.org/2000/svg" width="1080" height="{height}" viewBox="0 0 1080 {height}" role="img" aria-labelledby="title desc">
<title id="title">Rectangle union: sweep-line progression</title>
<desc id="desc">Panels show coverage after processing all events at each selected X coordinate. Blue bars indicate covered Y intervals. Orange marks the preceding slab.</desc>
<style>text{{font-family:Arial,sans-serif;fill:#17243a}} .small{{font-size:12px}} .label{{font-size:15px}}</style>
<rect width="1080" height="{height}" fill="#eef3f8"/>
<text x="24" y="34" font-size="24" font-weight="bold">Rectangle union / sweep-line progression</text>
<text x="24" y="61" class="label">Union: {} | Individual sum: {} | Overlap savings: {}</text>
<text x="24" y="84" class="small">After X-events: blue rectangles = active; blue side bars = covered Y; orange = preceding slab.</text>
<text x="24" y="104" class="small">Y increases upward. Event groups may be sampled; each slab refers to the immediately preceding event.</text>
"##,
        metrics.union_area, metrics.sum_individual_areas, metrics.overlap_savings
    );
    if snapshots.is_empty() {
        svg.push_str(r#"<text x="30" y="165" class="label">No nonzero-area rectangles. Union area = 0.</text>"#);
    }
    for (i, s) in snapshots.iter().enumerate() {
        let ox = 20 + i % 2 * 530;
        let oy = 120 + i / 2 * 370;
        writeln!(
            svg,
            r##"<g transform="translate({ox},{oy})">
<rect width="510" height="350" rx="10" fill="white" stroke="#d7e1ec"/>
<text x="16" y="25" font-size="17" font-weight="bold">Frame {}: x = {}</text>
<text x="16" y="47" class="small">Accumulated area: {}</text>
<rect x="34" y="85" width="380" height="175" fill="#f8fafc" stroke="#a9b6c8"/>
<rect x="{:.3}" y="85" width="{:.3}" height="175" fill="#f59e0b" fill-opacity=".18"/>
"##,
            i + 1,
            s.x,
            s.accumulated_area,
            px(s.previous_x),
            px(s.x) - px(s.previous_x)
        )
        .unwrap();
        for r in &nonempty {
            let active = r.x1 <= s.x && s.x < r.x2;
            let color = if active { "#2563eb" } else { "#64748b" };
            let opacity = if active { ".20" } else { ".06" };
            writeln!(svg, r#"<rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" fill="{color}" fill-opacity="{opacity}" stroke="{color}" stroke-width="1.2"/>"#,
                px(r.x1), py(r.y2), px(r.x2)-px(r.x1), py(r.y1)-py(r.y2)).unwrap();
        }
        writeln!(
            svg,
            r##"<line x1="{:.3}" y1="76" x2="{:.3}" y2="268" stroke="#e11d48" stroke-width="2"/>
<text x="433" y="75" class="small">Y cover</text>"##,
            px(s.x),
            px(s.x)
        )
        .unwrap();
        for &(a, b) in &s.covered_intervals {
            writeln!(
                svg,
                r##"<rect x="450" y="{:.3}" width="16" height="{:.3}" fill="#2563eb"/>"##,
                py(b),
                py(a) - py(b)
            )
            .unwrap();
        }
        let covered: u128 = s.covered_intervals.iter().map(|&(a, b)| length(a, b)).sum();
        writeln!(
            svg,
            r#"<text x="16" y="290" class="small">Preceding slab [{}, {}): area {}</text>
<text x="16" y="310" class="small">Covered Y-length for next slab: {covered}</text>
<text x="16" y="330" font-size="10">Bounds: x [{min_x}, {max_x}], y [{min_y}, {max_y}]</text>
</g>"#,
            s.previous_x, s.x, s.slab_area
        )
        .unwrap();
    }
    svg.push_str("</svg>\n");
    Ok(svg)
}
