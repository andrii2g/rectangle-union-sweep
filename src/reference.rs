use crate::{Rectangle, Result, geometry::length};

pub const VERIFICATION_LIMIT: usize = 100;

/// Independent O(n^3) reference algorithm; never uses sweep events or a tree.
pub fn reference_area(rectangles: &[Rectangle]) -> Result<u128> {
    if rectangles.len() > VERIFICATION_LIMIT {
        return Err(format!(
            "verification is limited to {VERIFICATION_LIMIT} rectangles"
        ));
    }
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for r in rectangles {
        r.validate()?;
        if !r.is_empty() {
            xs.extend([r.x1, r.x2]);
            ys.extend([r.y1, r.y2]);
        }
    }
    xs.sort_unstable();
    xs.dedup();
    ys.sort_unstable();
    ys.dedup();
    let mut area = 0u128;
    for x in xs.windows(2) {
        for y in ys.windows(2) {
            if rectangles
                .iter()
                .any(|r| r.x1 <= x[0] && x[1] <= r.x2 && r.y1 <= y[0] && y[1] <= r.y2)
            {
                area = area
                    .checked_add(length(x[0], x[1]) * length(y[0], y[1]))
                    .ok_or("reference area exceeds u128")?;
            }
        }
    }
    Ok(area)
}
