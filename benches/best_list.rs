#[allow(unused)]
use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rust_hex::best_list::BestList;

fn best_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("Best list");
    let lists: Vec<Vec<_>> = (1..=6)
        .map(|s| {
            (0..usize::pow(10, s))
                .map(|_| rand::random::<i32>())
                .collect()
        })
        .collect();

    fn bests_1(list: &[i32], n: usize) -> Vec<i32> {
        let mut sorted: Vec<_> = list.iter().cloned().collect();
        sorted.sort();
        sorted.iter().take(n).cloned().collect()
    }

    fn bests_2(list: &[i32], n: usize) -> Vec<i32> {
        let best_list = BestList::from_vec(&list, n);

        best_list.into()
    }

    for v in lists.iter() {
        group.bench_with_input(BenchmarkId::new("sort + take", v.len()), v, |b, v| {
            b.iter(|| bests_1(&v, 10))
        });
        group.bench_with_input(BenchmarkId::new("BestList", v.len()), v, |b, v| {
            b.iter(|| bests_2(&v, 10))
        });
    }
    group.finish();
}

criterion_group!(benches, best_list);
criterion_main!(benches);
