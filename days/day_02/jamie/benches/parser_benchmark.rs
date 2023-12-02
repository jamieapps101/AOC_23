use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use day_02_lib::GameLoader;

fn criterion_benchmark(c: &mut Criterion) {
    let mut data_path = std::path::PathBuf::from("days/day_02/jamie/data/input.txt");
    if !data_path.exists() {
        data_path = std::path::PathBuf::from("data/input.txt");
    }
    let data = std::fs::read_to_string(data_path)
        .expect("Could not open file")
        .lines()
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();
    let mut group = c.benchmark_group("Parsers");
    group.bench_with_input(
        BenchmarkId::new("base", "challenge input data"),
        &data,
        |b, d| {
            b.iter(|| {
                let mut loader: GameLoader<_, _> = d.iter().into();
                let mut r = Vec::with_capacity(100);
                while let Some(g) = loader.next_game() {
                    r.push(g)
                }
                assert_eq!(r.len(), 100);
            })
        },
    );
    group.bench_with_input(
        BenchmarkId::new("faster", "challenge input data"),
        &data,
        |b, d| {
            b.iter(|| {
                let mut loader: GameLoader<_, _> = d.iter().into();
                let mut r = Vec::with_capacity(100);
                while let Some(g) = loader.next_game_fast() {
                    r.push(g)
                }
                assert_eq!(r.len(), 100);
            })
        },
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
