use num_traits::{Float, PrimInt};
use std::{
    fmt::Debug,
    mem::{size_of, transmute},
};

/**
 * ====================================================================================================
 * Fixed parameters for the radix sort algorithms. Byte sized buckets are used, which is fairly
 * typical in radix sort implementations from what I can tell.
 * ====================================================================================================
 */
const MASK: u8 = 0xff;
const SHIFT_BITS: usize = 8;
const NUM_BUCKETS: usize = 256;
// Negative values are contained in the latter 128 bins. Recall that 127 = 0x7f = 0111 1111.
const FIRST_NEG_BUCKET: usize = NUM_BUCKETS / 2;

/**
 * ====================================================================================================
 * The following trait defines how a type should be chunked into bytes for the sake of binning. For
 * example, a u32 has 4 byte chunks, which will correspond to 4 iterations in the radix sort
 * algorithm.
 * ====================================================================================================
 */
pub trait Radix {
    fn to_radix(&self, offset: u8) -> usize;
}

#[macro_export]
macro_rules! radix_for_type {
    ($type:ty) => {
        impl Radix for $type {
            #[inline]
            fn to_radix(&self, offset: u8) -> usize {
                let offset = offset as $type;
                let mask = MASK as $type;
                (((mask << offset) & *self) >> offset) as usize
            }
        }
    };
}

#[macro_export]
macro_rules! radix_for_type_with_transmute {
    ($type_from:ty, $type_into:ty) => {
        impl Radix for $type_from {
            #[inline]
            fn to_radix(&self, offset: u8) -> usize {
                unsafe { transmute::<$type_from, $type_into>(*self) }.to_radix(offset)
            }
        }
    };
}

// Generate trait implementations for the following unsigned types. The core radix sort only
// operates on these types.
radix_for_type!(u32);
radix_for_type!(u64);
radix_for_type!(u128);
// Generate trait implementations for the following types that can be cast into the specified
// unsigned type. 
radix_for_type_with_transmute!(i32, u32);
radix_for_type_with_transmute!(i64, u64);
radix_for_type_with_transmute!(i128, u128);
radix_for_type_with_transmute!(f32, u32);
radix_for_type_with_transmute!(f64, u64);

/**
 * ====================================================================================================
 * Implementations of radix sort for vectors of types that implement Radix. There are separate
 * versions for integer and floating point types because I'm not sure how to accomplish it with one
 * trait in Rust yet (I'm what's colloquially referred to as a "noob").
 * ==================================================================================================== 
 */
pub trait RadixSortInt {
    fn radix_sort(&mut self);
}
impl<I> RadixSortInt for Vec<I>
where
    I: PrimInt + Radix + Debug,
{
    fn radix_sort(&mut self) {
        radix_sort_int(self)
    }
}

pub trait RadixSortFloat {
    fn radix_sort(&mut self);
}
impl<F> RadixSortFloat for Vec<F>
where
    F: Float + Radix + Debug,
{
    fn radix_sort(&mut self) {
        radix_sort_float(self)
    }
}

/**
 * ====================================================================================================
 * Core implementations of radix sort. These are all written to use byte sized buckets. This
 * particular implementation is the LSB variant, which allows for straightforward sorting of
 * arbitrarily sized data types. 
 * ====================================================================================================
 */
/**
 * This implementation is for floating point numbers using byte sizes buckets. It handles negative values.
 */
pub fn radix_sort_float<T>(vals: &mut Vec<T>)
where
    T: Radix + Clone + Copy + Debug,
{
    let num_iters = size_of::<T>();
    let mut counts: [usize; NUM_BUCKETS] = [0; NUM_BUCKETS];
    let mut offsets: [usize; NUM_BUCKETS] = [0; NUM_BUCKETS];
    let mut vals_buf: Vec<T> = vals.clone();

    for n_iter in 0..num_iters {
        let shift_bits = (n_iter * SHIFT_BITS) as u8;

        counts.iter_mut().for_each(|c| *c = 0);
        vals.iter()
            .map(|n| n.to_radix(shift_bits))
            .for_each(|b| counts[b] += 1);

        // Negative values only need to be handled on the final iteration. This is because the sign
        // bit is always MSB.
        if n_iter == num_iters - 1 {
            offsets[0] = counts[FIRST_NEG_BUCKET..].iter().sum();
            (1..FIRST_NEG_BUCKET).for_each(|i| offsets[i] = offsets[i - 1] + counts[i - 1]);

            // Reverse order prefix sum to fix the ordering for negative values.
            offsets[NUM_BUCKETS-1] = 0;
            (0..FIRST_NEG_BUCKET-1).for_each(|i| offsets[NUM_BUCKETS-2-i] = offsets[NUM_BUCKETS-1-i] + counts[NUM_BUCKETS-1-i]);

            // Fix positioning of negative values.
            (FIRST_NEG_BUCKET..NUM_BUCKETS).for_each(|i| offsets[i] += counts[i]);

        } else {
            offsets[0] = 0;
            (1..NUM_BUCKETS).for_each(|i| offsets[i] = offsets[i - 1] + counts[i - 1]);
        }

        for n in &vals_buf {
            let b = n.to_radix(shift_bits);
            if (n_iter == num_iters - 1) && (b >= FIRST_NEG_BUCKET) {
                offsets[b] -= 1;
                unsafe {
                    *vals.get_unchecked_mut(*offsets.get_unchecked(b)) = *n;
                }
            } else {
                unsafe {
                    *vals.get_unchecked_mut(*offsets.get_unchecked(b)) = *n;
                }
                offsets[b] += 1;
            }
        }
        vals_buf.copy_from_slice(&vals);
    }
}

/**
 * This implementation is for radix sorting integer types using byte sized buckets. It handles
 * negative values.
 */
pub fn radix_sort_int<T>(vals: &mut Vec<T>)
where
    T: Radix + Clone + Copy + Debug,
{
    let num_iters = size_of::<T>();
    let mut counts: [usize; NUM_BUCKETS] = [0; NUM_BUCKETS];
    let mut offsets: [usize; NUM_BUCKETS] = [0; NUM_BUCKETS];
    let mut vals_buf: Vec<T> = vals.clone();

    for n_iter in 0..num_iters {
        let shift_bits = (n_iter * SHIFT_BITS) as u8;

        counts.iter_mut().for_each(|c| *c = 0);
        vals.iter()
            .map(|n| n.to_radix(shift_bits))
            .for_each(|b| counts[b] += 1);

        // Negative values only need to be handled on the final iteration. This is because the sign
        // bit is always MSB.
        if n_iter == num_iters - 1 {
            // Negative values are contained in the latter 128 bins. Recall that 127 = 0x7f = 0111 1111.
            offsets[0] = counts[FIRST_NEG_BUCKET..].iter().sum();
            (1..FIRST_NEG_BUCKET).for_each(|i| offsets[i] = offsets[i - 1] + counts[i - 1]);

            // Reverse order prefix sum to fix the ordering for negative values.
            offsets[FIRST_NEG_BUCKET] = 0;
            (FIRST_NEG_BUCKET+1 .. NUM_BUCKETS).for_each(|i| offsets[i] = offsets[i - 1] + counts[i - 1]);

        } else {
            offsets[0] = 0;
            (1..NUM_BUCKETS).for_each(|i| offsets[i] = offsets[i - 1] + counts[i - 1]);
        }

        for n in &vals_buf {
            let b = n.to_radix(shift_bits);
            unsafe {
                *vals.get_unchecked_mut(*offsets.get_unchecked(b)) = *n;
            }
            offsets[b] += 1;
        }
        vals_buf.copy_from_slice(&vals);
    }
}

/**
 * ====================================================================================================
 * Tests.
 * ====================================================================================================
 */
#[cfg(test)]
mod tests {
    use crate::{RadixSortFloat, RadixSortInt};

    #[test]
    fn test_sort_i32() {
        let mut vals: Vec<i32> = (-512..512).rev().collect();
        let expected: Vec<i32> = (-512..512).collect();
        vals.radix_sort();
        assert_eq!(expected, vals);
    }

    #[test]
    fn test_sort_i64() {
        let mut vals: Vec<i64> = vec![
            i32::max_value() as i64 + 1,
            4,
            3,
            2,
            1,
            -1,
            -2,
            -3,
            -4,
            i32::min_value() as i64 - 1,
        ];
        let expected: Vec<i64> = vec![
            i32::min_value() as i64 - 1,
            -4,
            -3,
            -2,
            -1,
            1,
            2,
            3,
            4,
            i32::max_value() as i64 + 1,
        ];
        vals.radix_sort();
        assert_eq!(expected, vals);
    }

    #[test]
    fn test_sort_u32() {
        let mut vals: Vec<u32> = (0..1024).rev().collect();
        let expected: Vec<u32> = (0..1024).collect();
        vals.radix_sort();
        assert_eq!(expected, vals);
    }

    #[test]
    fn test_sort_u64() {
        let mut vals: Vec<u64> = vec![u32::max_value() as u64 + 1, 4, 3, 2, 1];
        let expected: Vec<u64> = vec![1, 2, 3, 4, u32::max_value() as u64 + 1];
        vals.radix_sort();
        assert_eq!(expected, vals);
    }

    #[test]
    fn test_sort_u128() {
        let mut vals: Vec<u128> = vec![u64::max_value() as u128 + 1, 4, 3, 2, 1];
        let expected: Vec<u128> = vec![1, 2, 3, 4, u64::max_value() as u128 + 1];
        vals.radix_sort();
        assert_eq!(expected, vals);
    }

    #[test]
    fn test_sort_f32() {
        let mut vals: Vec<f32> = (0..1024).rev().map(|n| n as f32).collect();
        let expected: Vec<f32> = (0..1024).map(|n| n as f32).collect();
        vals.radix_sort();
        assert_eq!(expected, vals);
    }

    #[test]
    fn test_sort_f64() {
        let mut vals: Vec<f64> = vec![
            f32::MAX as f64 + 1f64,
            4f64,
            3f64,
            2f64,
            1f64,
            -1f64,
            -2f64,
            -3f64,
            -4f64,
            f32::MIN as f64 - 1f64,
        ];
        let expected: Vec<f64> = vec![
            f32::MIN as f64 - 1f64,
            -4f64,
            -3f64,
            -2f64,
            -1f64,
            1f64,
            2f64,
            3f64,
            4f64,
            f32::MAX as f64 + 1f64,
        ];
        vals.radix_sort();
        assert_eq!(expected, vals);
    }
}
