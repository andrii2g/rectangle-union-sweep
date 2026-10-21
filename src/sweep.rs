use crate::{Rectangle, Result, geometry::length, segment_tree::SegmentTree};
use serde::Serialize;
use std::time::Instant;

/// Areas are decimal strings in JSON to avoid precision loss in JavaScript consumers.
#[derive(Debug, Serialize)]
pub struct Metrics {
    pub union_area: String,
    pub sum_individual_areas: String,
    pub overlap_savings: String,
    pub events_processed: usize,
    pub segment_tree_updates: usize,
    pub tree_nodes_visited: usize,
    pub maximum_tree_depth: usize,
    pub runtime_ns: u128,
}

#[derive(Debug)]
pub struct Snapshot {
    pub x: i64,
    pub previous_x: i64,
    pub slab_area: u128,
    pub accumulated_area: u128,
    /// Coverage after applying every event at x; applies to the following slab.
    pub covered_intervals: Vec<(i64, i64)>,
}

#[derive(Clone, Copy)]
struct Event {
    x: i64,
    start: usize,
    end: usize,
    entering: bool,
}

pub fn union_area(rectangles: &[Rectangle]) -> Result<Metrics> {
    union_area_with_trace(rectangles, 0).map(|(metrics, _)| metrics)
}

/// Retains at most max_snapshots evenly spaced event groups, including endpoints
/// when the limit is at least two. Trace extraction is included in runtime.
pub fn union_area_with_trace(
    rectangles: &[Rectangle],
    max_snapshots: usize,
) -> Result<(Metrics, Vec<Snapshot>)> {
    let started = Instant::now();
    let mut sum = 0u128;
    let mut ys = Vec::new();
    for (index, r) in rectangles.iter().enumerate() {
        sum = sum
            .checked_add(r.area().map_err(|e| format!("rectangle {index}: {e}"))?)
            .ok_or("sum of individual areas exceeds u128")?;
        if !r.is_empty() {
            ys.extend([r.y1, r.y2]);
        }
    }
    ys.sort_unstable();
    ys.dedup();
    let mut events = Vec::new();
    for r in rectangles.iter().filter(|r| !r.is_empty()) {
        let start = ys.binary_search(&r.y1).unwrap();
        let end = ys.binary_search(&r.y2).unwrap();
        events.push(Event {
            x: r.x1,
            start,
            end,
            entering: true,
        });
        events.push(Event {
            x: r.x2,
            start,
            end,
            entering: false,
        });
    }
    events.sort_unstable_by_key(|e| e.x);
    let groups = events
        .iter()
        .enumerate()
        .filter(|(i, e)| *i == 0 || events[*i - 1].x != e.x)
        .count();
    let sample_count = max_snapshots.min(groups);
    let selected: Vec<usize> = (0..sample_count)
        .map(|i| {
            if sample_count == 1 {
                groups - 1
            } else {
                ((i as u128 * (groups - 1) as u128) / (sample_count - 1) as u128) as usize
            }
        })
        .collect();
    let mut snapshots = Vec::new();
    let mut area = 0u128;
    let mut visits = 0;
    let mut max_depth = 0;
    if let Some(first) = events.first() {
        let mut tree = SegmentTree::new(&ys);
        let mut previous_x = first.x;
        let mut cursor = 0;
        let mut group = 0;
        while cursor < events.len() {
            let x = events[cursor].x;
            let slab_area = tree.covered() * length(previous_x, x);
            area = area
                .checked_add(slab_area)
                .ok_or("union area exceeds u128")?;
            while cursor < events.len() && events[cursor].x == x {
                let e = events[cursor];
                tree.update(e.start, e.end, e.entering);
                cursor += 1;
            }
            if selected.get(snapshots.len()) == Some(&group) {
                snapshots.push(Snapshot {
                    x,
                    previous_x,
                    slab_area,
                    accumulated_area: area,
                    covered_intervals: tree.intervals(),
                });
            }
            previous_x = x;
            group += 1;
        }
        visits = tree.visits;
        max_depth = tree.max_depth;
    }
    Ok((
        Metrics {
            union_area: area.to_string(),
            sum_individual_areas: sum.to_string(),
            overlap_savings: (sum - area).to_string(),
            events_processed: events.len(),
            segment_tree_updates: events.len(),
            tree_nodes_visited: visits,
            maximum_tree_depth: max_depth,
            runtime_ns: started.elapsed().as_nanos(),
        },
        snapshots,
    ))
}
