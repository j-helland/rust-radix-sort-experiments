use rand::distributions::{Distribution, Uniform, uniform::SampleUniform};
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use itertools::Itertools;

use radix::radix_sort_inplace;

//const NUM_BITS: u8 = 8;
//const NUM_BINS: usize = 1 << NUM_BITS;
//const MASK: u32 = 0xff;
//const LO_BIT: u32 = 0;

fn sample_nums<R: Rng, N: SampleUniform>(rng: &mut R, dist: &Uniform<N>, num_samples: usize) -> Vec<N> {
    (0..num_samples).map(|_| dist.sample(rng)).collect_vec()
}

fn main() {
    let num_samples: usize = 8;
    let max_val: u32 = u8::max_value() as u32;

    let mut rng: StdRng = StdRng::seed_from_u64(0);
    let dist: Uniform<u32> = Uniform::new(0, max_val);
    let mut nums = sample_nums(&mut rng, &dist, num_samples);

    println!();
    println!("ori:{:?}", nums);
    radix_sort_inplace(max_val, &mut nums);
    println!("res:{:?}", nums);
    nums.sort();
    println!("exp:{:?}", nums);
}

#[cfg(test)]
mod tests {
    use crate::radix_sort_inplace;

    #[test]
    fn test_sort() {
        let mut vals: Vec<u32> = vec![4, 3, 2, 1];
        let expected: Vec<u32> = vec![1, 2, 3, 4];
        radix_sort_inplace(u32::max_value(), &mut vals);
        assert_eq!(expected, vals);
    }
}
