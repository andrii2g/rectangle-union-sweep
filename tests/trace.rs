use rectangle_union_sweep::{Dataset, render_svg, union_area_with_trace};
fn sample() -> Dataset {
    serde_json::from_str(include_str!("../examples/overlapping.json")).unwrap()
}
#[test]
fn trace_tracks_post_event_coverage_and_preceding_slabs() {
    let (m, trace) = union_area_with_trace(&sample().rectangles, 24).unwrap();
    assert_eq!(m.union_area, "48");
    assert_eq!(trace.iter().map(|s| s.x).collect::<Vec<_>>(), [0, 3, 6, 9]);
    assert_eq!(
        trace.iter().map(|s| s.slab_area).collect::<Vec<_>>(),
        [0, 12, 21, 15]
    );
    assert_eq!(trace[1].covered_intervals, [(0, 7)]);
    assert!(trace[3].covered_intervals.is_empty());
    let (_, sampled) = union_area_with_trace(&sample().rectangles, 2).unwrap();
    assert_eq!(sampled.iter().map(|s| s.x).collect::<Vec<_>>(), [0, 9]);
    let (_, single) = union_area_with_trace(&sample().rectangles, 1).unwrap();
    assert_eq!(single[0].x, 9);
}
#[test]
fn svg_handles_empty_input_and_limits() {
    assert!(render_svg(&[], 4).unwrap().contains("Union area = 0"));
    assert!(
        render_svg(&sample().rectangles, 4)
            .unwrap()
            .contains("area 21")
    );
    assert!(render_svg(&[], 0).is_err());
    assert!(render_svg(&[], 25).is_err());
    assert!(render_svg(&vec![sample().rectangles[0]; 201], 4).is_err());
}
