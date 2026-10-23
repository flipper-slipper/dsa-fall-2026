use criterion::{criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration, SamplingMode};
use dynamic_programming::matrix::SquareMatrix;
use rand::{rngs::StdRng, RngExt, SeedableRng};

/// Makes an `n` x `n` matrix of random values between 0 and 1.
fn make_input(n: usize, seed: u64) -> SquareMatrix {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut matrix = SquareMatrix::new(n);
    for row in 0..n {
        for col in 0..n {
            matrix.set(row, col, rng.random());
        }
    }
    matrix
}

fn bench_multiply(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix_multiply");
    group.sample_size(10);
    group.sampling_mode(SamplingMode::Flat);
    group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    for n in [4, 16, 64, 256, 512, 1024, 2048] {
        let a = make_input(n, 1);
        let b = make_input(n, 2);

        group.bench_function(BenchmarkId::new("multiply", n), |bench| {
            bench.iter(|| a.multiply(&b))
        });
        group.bench_function(BenchmarkId::new("strassen_multiply", n), |bench| {
            bench.iter(|| a.strassen_multiply(&b))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_multiply);
criterion_main!(benches);
