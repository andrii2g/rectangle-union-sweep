use rectangle_union_sweep::{Rectangle as R, union_area};
fn r(x1: i64, y1: i64, x2: i64, y2: i64) -> R {
    R { x1, y1, x2, y2 }
}
fn area(rs: &[R]) -> u128 {
    union_area(rs).unwrap().union_area.parse().unwrap()
}

#[test]
fn known_areas() {
    assert_eq!(area(&[]), 0);
    assert_eq!(area(&[r(0, 0, 6, 4), r(3, 2, 9, 7)]), 48);
    assert_eq!(area(&[r(0, 0, 6, 4), r(0, 0, 6, 4)]), 24);
    assert_eq!(area(&[r(-5, -5, 5, 5), r(-1, -1, 1, 1)]), 100);
    assert_eq!(area(&[r(0, 0, 2, 2), r(4, 4, 6, 6)]), 8);
    assert_eq!(area(&[r(0, 0, 2, 2), r(2, 0, 4, 2), r(4, 2, 6, 4)]), 12);
    assert_eq!(area(&[r(1, 1, 1, 8), r(0, 2, 7, 2)]), 0);
}
#[test]
fn invalid_and_extreme_coordinates() {
    assert!(union_area(&[r(5, 0, 1, 2)]).is_err());
    let rect = r(i64::MIN, i64::MIN, i64::MAX, i64::MAX);
    assert_eq!(area(&[rect]), (u64::MAX as u128).pow(2));
    assert!(union_area(&[rect, rect]).unwrap_err().contains("sum"));
}
#[test]
fn counters_ignore_empty_rectangles() {
    let m = union_area(&[r(0, 0, 1, 1), r(0, 0, 0, 5)]).unwrap();
    assert_eq!(m.events_processed, 2);
    assert_eq!(m.segment_tree_updates, 2);
    assert_eq!(m.maximum_tree_depth, 0);
}
