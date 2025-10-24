use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

use kindergarten_garden::plants;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("plants 20", |b| {
        b.iter(|| {
            plants(
                black_box(
                    "VRCGVVRVCGGCCGVRGCVCGCGV
VRCCCGCRRGVCGCRVVCVGCGCV",
                ),
                black_box("Larry"),
            )
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
