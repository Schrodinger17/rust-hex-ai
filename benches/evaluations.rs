#[allow(unused)]
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rust_hex::{
    board::Board,
    evaluation::{Evaluation, Evaluation1, Evaluation2},
};

fn evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Evaluation");
    let boards = [
        Board::new(),
        Board::random_board(10),
        Board::random_board(20),
        Board::random_board(30),
        Board::random_board(40),
    ];

    let evaluation1 = Evaluation1::new();
    let evaluation2 = Evaluation2::new();

    for (i, v) in boards.iter().enumerate() {
        group.bench_with_input(BenchmarkId::new("Evaluation1", i), v, |b, v| {
            b.iter(|| Evaluation::score(&evaluation1, v))
        });
        group.bench_with_input(BenchmarkId::new("Evaluation2", i), v, |b, v| {
            b.iter(|| Evaluation::score(&evaluation2, v))
        });
    }
    group.finish();
}

criterion_group!(benches, evaluation);
criterion_main!(benches);
