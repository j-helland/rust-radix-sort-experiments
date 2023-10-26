use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rand::distributions::Uniform;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rdxsort::{self, RdxSort};

use radix::{RadixSortInt, RadixSortFloat};

const NUM_SAMPLES: [usize; 5] = [1024, 2048, 10000, 1000000, 10000000];

macro_rules! benchmark_type {
    ($fn_bench:ident, $t:ty, $min:expr, $max:expr) => {
        fn $fn_bench(c: &mut Criterion) {
            // Generate test data.
            let generate_data = |num_samples| {
                let rng: StdRng = StdRng::seed_from_u64(0);
                let dist: Uniform<$t> = Uniform::new($min, $max);
                let vals: Vec<$t> = rng.sample_iter(dist).take(num_samples).collect();
                vals
            };

            let mut group = c.benchmark_group(format!("radix_{}", stringify!($t)));
            for num_samples in NUM_SAMPLES {
                let vals = generate_data(num_samples);
                group.bench_with_input(
                    BenchmarkId::from_parameter(num_samples),
                    &vals,
                    |b, vals| {
                        let mut vals = vals.clone();
                        b.iter(|| vals.radix_sort());
                    },
                );
            }
            group.finish();

            let mut group = c.benchmark_group(format!("rdxsort_{}", stringify!($t)));
            for num_samples in NUM_SAMPLES {
                let vals = generate_data(num_samples);
                group.bench_with_input(
                    BenchmarkId::from_parameter(num_samples),
                    &vals,
                    |b, vals| {
                        let mut vals = vals.clone();
                        b.iter(|| vals.rdxsort());
                    },
                );
            }
            group.finish();

            let mut group = c.benchmark_group(format!("std_quicksort_{}", stringify!($t)));
            for num_samples in NUM_SAMPLES {
                let vals = generate_data(num_samples);
                group.bench_with_input(
                    BenchmarkId::from_parameter(num_samples),
                    &vals,
                    |b, vals| {
                        let mut vals = vals.clone();
                        b.iter(|| vals.sort_by(|a, b| a.partial_cmp(b).unwrap()));
                    },
                );
            }
            group.finish();
        }
    };
}

benchmark_type!(bench_u32, u32, 0, u32::max_value());
benchmark_type!(bench_u64, u64, 0, u64::max_value());
benchmark_type!(bench_i32, i32, i32::min_value(), i32::max_value());
benchmark_type!(bench_i64, i64, i64::min_value(), i64::max_value());
benchmark_type!(bench_f32, f32, -1.0, 1.0);
benchmark_type!(bench_f64, f64, -1.0, 1.0);

criterion_group!(
    benches,
    bench_u32,
    bench_u64,
    bench_i32,
    bench_i64,
    bench_f32,
    bench_f64,
);
criterion_main!(benches);

