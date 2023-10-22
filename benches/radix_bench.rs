use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};

use rand::distributions::{Distribution, Uniform, uniform::SampleUniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use itertools::Itertools;

use radix::radix_sort_inplace;

fn sample_nums<R: Rng, N: SampleUniform>(rng: &mut R, dist: &Uniform<N>, num_samples: usize) -> Vec<N> {
    (0..num_samples).map(|_| dist.sample(rng)).collect_vec()
}

fn bench_radix_sort_inplace_uniform(c: &mut Criterion) {
    let mut rng: StdRng = StdRng::seed_from_u64(0);
    let dist: Uniform<u32> = Uniform::new_inclusive(0, u32::max_value());

    let mut group = c.benchmark_group("radix_sequential_uniform");
    for num_samples in [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048].iter() {
        let vals: Vec<u32> = sample_nums(&mut rng, &dist, *num_samples as usize);
        group.bench_with_input(BenchmarkId::from_parameter(num_samples), &vals, |b, vals| {
            let mut vals = vals.clone();
            b.iter(|| radix_sort_inplace(u32::max_value(), &mut vals));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_radix_sort_inplace_uniform);
criterion_main!(benches);

