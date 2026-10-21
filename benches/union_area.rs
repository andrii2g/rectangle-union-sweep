use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use rectangle_union_sweep::{Pattern, generate, union_area};
use std::hint::black_box;
fn benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("union_area");
    group.sample_size(10);
    for pattern in [
        Pattern::Overlapping,
        Pattern::Nested,
        Pattern::Disjoint,
        Pattern::Adjacent,
    ] {
        for count in [1_000, 10_000, 100_000] {
            let dataset = generate(pattern, count, 42).unwrap();
            group.bench_with_input(
                BenchmarkId::new(format!("{pattern:?}"), count),
                &dataset,
                |b, ds| {
                    b.iter(|| union_area(black_box(&ds.rectangles)).unwrap());
                },
            );
        }
    }
    group.finish();
}
criterion_group!(benches, benchmarks);
criterion_main!(benches);
