# rectangle-union-sweep

Exact union area of axis-aligned integer rectangles in **O(n log n)** time and **O(n)** memory, using an X sweep, Y-coordinate compression, and a coverage segment tree.

## Quick start

Install a current stable Rust toolchain (Rust 2024 edition; validated with Rust 1.97.1).

```sh
cargo run --release -- area examples/overlapping.json
cargo run --release -- verify examples/overlapping.json
cargo run --release -- generate --pattern overlapping --count 10000 --seed 42 --output rectangles.json
cargo run --release -- area rectangles.json
cargo run --release -- render examples/overlapping.json --output sweep.svg --frames 8
```

Open the SVG in a browser. See [the example progression](examples/sweep.svg).

## Input and output

```json
{
  "rectangles": [
    {"x1": 0, "y1": 0, "x2": 6, "y2": 4},
    {"x1": 3, "y1": 2, "x2": 9, "y2": 7}
  ]
}
```

Coordinates are signed 64-bit integers. Reversed bounds are rejected; zero-area rectangles are accepted and generate no events. Unknown JSON fields are rejected. Rectangles represent half-open regions `[x1, x2) × [y1, y2)`, so shared edges add no area.

The example has union area **48**, individual sum **54**, and overlap savings **6**. Area quantities are exact `u128` values internally and decimal strings in JSON, avoiding JavaScript integer precision loss. Coordinate differences are calculated in `i128`. An overflowing individual-area sum returns an error, even if its union would fit.

The CLI prints errors to stderr and exits unsuccessfully on invalid input, verification failure, or I/O errors.

## Algorithm

1. Validate rectangles and collect unique sorted Y boundaries.
2. Represent each nonempty rectangle by a left-edge insertion and right-edge removal.
3. Sort these events by X and process equal-X events as one group.
4. Before applying each group, accumulate `covered_y_length × (x - previous_x)`.
5. Update the segment tree over compressed Y intervals.

Tree leaves represent gaps between coordinates, not coordinates themselves. Each node stores a cover count and covered length. A positive count covers its entire geometric span; otherwise its covered length is the sum of its children (or zero at an uncovered leaf). The root gives the active covered Y-length in constant time. An interval update visits O(log n) nodes.

## Generators and verification

`generate` supports `overlapping`, `nested`, `disjoint`, and `adjacent`. The default seed is 42; the explicit SplitMix64 generator gives stable results across runs and platforms. Overlapping rectangles all contain the origin; nested rectangles shrink strictly; disjoint columns have gaps; adjacent columns share edges. The seed changes dimensions or placement depending on the pattern. Generation is capped at 1,000,000 rectangles.

`verify` independently enumerates elementary cells formed by all X and Y boundaries, tests each cell against the input rectangles, and sums covered areas. It does not use sweep events or a segment tree. Its direct O(n³) implementation is capped at 100 input rectangles.

## SVG progression

`render` accepts up to 200 rectangles and 1–24 frames (default 8). Frames sample event groups evenly; limits of at least two retain the first and last groups. A single frame shows the final group.

Each panel shows the state **after** all events at X. Blue rectangles are active, blue side bars show merged covered Y intervals, the red line marks X, and orange marks the slab immediately preceding that event. Accumulated area includes all intervening slabs even when groups are skipped. Y increases upward. Empty input produces an explanatory diagram.

SVG coordinates are floating-point approximations for display only. Computation remains exact. Trace extraction and rendering add work beyond the O(n log n) area algorithm.

## Metrics

| Field | Meaning |
|---|---|
| `union_area` | Area covered at least once |
| `sum_individual_areas` | Sum before removing repeated coverage |
| `overlap_savings` | Individual sum minus union area |
| `events_processed` | Two per nonempty rectangle |
| `segment_tree_updates` | One interval update per event |
| `tree_nodes_visited` | Total nodes visited during interval updates |
| `maximum_tree_depth` | Deepest visited level; root is depth zero |
| `runtime_ns` | Validation, compression, sorting, and sweeping; excludes file I/O and serialization |

Overlap savings measures excess multiplicity: a region covered three times contributes twice its area. It is not the area covered by at least two rectangles. The library's optional traced variant includes trace extraction in its runtime.

## Library

```rust
use rectangle_union_sweep::{Rectangle, union_area};

let rectangles = [Rectangle { x1: 0, y1: 0, x2: 6, y2: 4 }];
let metrics = union_area(&rectangles)?;
assert_eq!(metrics.union_area, "24");
# Ok::<(), String>(())
```

Public entry points also include `reference_area`, `generate`, `union_area_with_trace`, and `render_svg`.

## Validation and benchmarks

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo test --release
cargo bench --bench union_area
```

Tests cover known areas, duplicates, nesting, shared boundaries, negative and extreme coordinates, zero-area rectangles, rejected bounds, overflow, trace semantics, CLI behavior, and 900 small randomized differential datasets with reorder/duplicate/translation invariants.

Criterion benchmarks cover each pattern at 1,000, 10,000, and 100,000 rectangles. Dataset generation is outside the timed loop; validation and all algorithm allocations are included. Results depend on the machine and input distribution. For a quick benchmark smoke check:

```sh
cargo bench --bench union_area -- --test
```

No GitHub Actions or workflows are included.
