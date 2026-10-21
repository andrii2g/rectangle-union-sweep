//! Exact O(n log n) union area for axis-aligned integer rectangles.
mod geometry;
mod segment_tree;
mod sweep;

pub use geometry::{Dataset, Rectangle};
pub use sweep::{Metrics, Snapshot, union_area, union_area_with_trace};

/// Errors include invalid bounds and numeric overflow; input is never normalized silently.
pub type Result<T> = std::result::Result<T, String>;
