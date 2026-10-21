use crate::{Dataset, Rectangle, Result};
use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Pattern {
    Overlapping,
    Nested,
    Disjoint,
    Adjacent,
}

/// SplitMix64 with explicitly wrapping arithmetic: stable across platforms and runs.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

/// All overlapping rectangles contain the origin. Nested rectangles have strictly
/// decreasing bounds. Disjoint and adjacent patterns use separated/touching columns.
pub fn generate(pattern: Pattern, count: usize, seed: u64) -> Result<Dataset> {
    if count > 1_000_000 {
        return Err("generation is limited to 1,000,000 rectangles".into());
    }
    let mut rng = Rng(seed);
    let offset = (rng.next() % 2001) as i64 - 1000;
    let mut rectangles = Vec::with_capacity(count);
    for i in 0..count {
        let i = i as i64;
        let r = match pattern {
            Pattern::Overlapping => Rectangle {
                x1: -1 - (rng.next() % 1000) as i64,
                y1: -1 - (rng.next() % 1000) as i64,
                x2: 1 + (rng.next() % 1000) as i64,
                y2: 1 + (rng.next() % 1000) as i64,
            },
            Pattern::Nested => {
                let radius = count as i64 - i;
                Rectangle {
                    x1: offset - radius,
                    y1: offset - radius,
                    x2: offset + radius,
                    y2: offset + radius,
                }
            }
            Pattern::Disjoint | Pattern::Adjacent => {
                let stride = if matches!(pattern, Pattern::Disjoint) {
                    12
                } else {
                    10
                };
                let x = offset + i * stride;
                Rectangle {
                    x1: x,
                    y1: offset,
                    x2: x + 10,
                    y2: offset + 10,
                }
            }
        };
        rectangles.push(r);
    }
    Ok(Dataset { rectangles })
}
