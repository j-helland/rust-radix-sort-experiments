pub fn radix_sort_inplace(max: u32, vals: &mut Vec<u32>) {
    let mut mask = 1;
    // We can terminate once the MSB is shifted out.
    while (mask > 0) && (mask <= max) {
        // Compute the prefix sum over the histogram of counts for the target bit.
        // We only need to actually count the number of 0 bits at this position since the rest of
        // the prefix sum values can be inferred from this value.
        let psum: u32 = vals.iter().map(|n| ((mask & n) == 0) as u32).sum();
        let psum = psum as usize;

        let buffer = vals.clone();

        // These values track the exclusive prefix sums over the values at the current bit
        // position. These are relative offsets into each bin, which ensure a stable sort.
        let mut offset_0: usize = 0;
        let mut offset_1: usize = 0;

        for n in &buffer {
            // Branchless computation of the final position for this value.
            let b = ((mask & *n) > 0) as usize;
            let pos = b * psum;
            let idx = pos as usize
                + b * offset_1 
                + (1 - b) * offset_0;

            vals[idx] = *n;

            // Branchless update of the relative offsets.
            offset_1 += b;
            offset_0 += 1 - b;
        }

        mask <<= 1;
    }
}
