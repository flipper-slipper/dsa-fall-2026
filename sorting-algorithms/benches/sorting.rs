use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use rand::{rngs::StdRng, RngExt, SeedableRng};
use sorting_algorithms::algorithms::{insertion_sort, merge_sort, quick_sort, selection_sort};

/// Max len for the O(n^2) sorts otherwise the computer might crash. 
const QUADRATIC_MAX_N: usize = 10_000;

/// Makes a Vector of random values for a desired length `n`
fn make_input(n: usize) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(42);
    (0..n).map(|_| rng.random()).collect()
}

/// Unit test helper method to verify sorting algorithms work. 
fn check_sorted(name: &str, sort: fn(&mut [u64]), input: &[u64]) {
    let mut actual = input.to_vec();
    sort(&mut actual);
    let mut expected = input.to_vec();
    expected.sort();
    assert!(actual.is_sorted(), "{name} output is not sorted (n = {})", input.len());
    assert_eq!(actual, expected, "{name} output does not match std sort (n = {})", input.len());
}

/// The main criterion function that does the benchmarking.
fn bench_sorts(c: &mut Criterion) {
    let mut group = c.benchmark_group("sorting");

    let sorts: [(&str, fn(&mut [u64]), usize); 4] = [
        ("selection_sort", selection_sort, QUADRATIC_MAX_N),
        ("insertion_sort", insertion_sort, QUADRATIC_MAX_N),
        ("quick_sort", quick_sort, usize::MAX),
        ("merge_sort", merge_sort, usize::MAX),
    ];

    for n in [10, 100, 1_000, 10_000, 1_000_000] {
        let input = make_input(n);
        // Large inputs are cloned one at a time so batch doesn't hold many copies in memory
        let batch_size = if n >= 1_000_000 { BatchSize::LargeInput } else { BatchSize::SmallInput };

        for (name, sort, max_n) in sorts {
            if n > max_n {
                continue;
            }
            check_sorted(name, sort, &input);
            group.bench_function(BenchmarkId::new(name, n), |b| {
                b.iter_batched(|| input.clone(), |mut data| sort(&mut data), batch_size)
            });
        }
    }

    group.finish();
}

criterion_group!(benches, bench_sorts);
criterion_main!(benches);
