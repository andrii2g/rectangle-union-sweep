use rectangle_union_sweep::{Pattern, Rectangle, generate, reference_area, union_area};
fn area(rs: &[Rectangle]) -> u128 {
    union_area(rs).unwrap().union_area.parse().unwrap()
}

#[test]
fn randomized_differential_and_invariants() {
    let mut state = 42u64;
    let mut next = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        ((state >> 32) % 31) as i64 - 15
    };
    for count in 0..30 {
        for _ in 0..30 {
            let mut rs = Vec::new();
            for _ in 0..count {
                let (a, b, c, d) = (next(), next(), next(), next());
                rs.push(Rectangle {
                    x1: a.min(b),
                    x2: a.max(b),
                    y1: c.min(d),
                    y2: c.max(d),
                });
            }
            let expected = reference_area(&rs).unwrap();
            assert!(expected <= rs.iter().map(|r| r.area().unwrap()).sum::<u128>());
            assert_eq!(area(&rs), expected);
            rs.reverse();
            assert_eq!(area(&rs), expected);
            if let Some(r) = rs.first().copied() {
                rs.push(r);
            }
            assert_eq!(area(&rs), expected);
            for r in &mut rs {
                r.x1 += 100;
                r.x2 += 100;
                r.y1 -= 100;
                r.y2 -= 100;
            }
            assert_eq!(area(&rs), expected);
        }
    }
}

#[test]
fn generators_are_reproducible_and_match_reference() {
    for pattern in [
        Pattern::Overlapping,
        Pattern::Nested,
        Pattern::Disjoint,
        Pattern::Adjacent,
    ] {
        let a = generate(pattern, 20, 42).unwrap();
        let b = generate(pattern, 20, 42).unwrap();
        assert_eq!(
            serde_json::to_string(&a).unwrap(),
            serde_json::to_string(&b).unwrap()
        );
        assert_eq!(area(&a.rectangles), reference_area(&a.rectangles).unwrap());
        assert!(generate(pattern, 0, 42).unwrap().rectangles.is_empty());
    }
    for pattern in [Pattern::Disjoint, Pattern::Adjacent] {
        assert_eq!(area(&generate(pattern, 20, 42).unwrap().rectangles), 2000);
    }
    assert_eq!(
        area(&generate(Pattern::Nested, 20, 42).unwrap().rectangles),
        1600
    );
}

#[test]
fn reference_limit_and_extremes() {
    let r = Rectangle {
        x1: i64::MIN,
        y1: i64::MIN,
        x2: i64::MAX,
        y2: i64::MAX,
    };
    assert_eq!(reference_area(&[r]).unwrap(), (u64::MAX as u128).pow(2));
    assert!(reference_area(&vec![r; 101]).is_err());
}
