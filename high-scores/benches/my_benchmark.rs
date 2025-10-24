use criterion::{Criterion, criterion_group, criterion_main};
use high_scores::HighScores;

fn personal_top_three_from_a_list_of_scores() {
    let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);

    assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("personal_top_three_from_a_list_of_scores", |b| {
        b.iter(|| personal_top_three_from_a_list_of_scores())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
