use radix::RadixSortFloat;

// Just a dumb script that I used for debugging.
fn main() {
    let mut nums: Vec<f64> = vec![
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
    let exp: Vec<f64> = vec![
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

    println!();
    println!("ori:{:?}", nums);
    nums.radix_sort();
    println!("res:{:?}", nums);
    println!("exp:{:?}", exp);
}


