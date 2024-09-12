use ark_ff::Field;
use rand::thread_rng;

// We need 2^v evaluation points

pub fn index_to_bools(mut index: usize, v: usize) -> Vec<bool> {
    let mut bool_vec = vec![false; v];
    let mut place = 0;
    assert!((index as f64).log2() < (v as f64));
    while index > 0 {
        bool_vec[place] = index & 1 == 1;
        place += 1;
        index >>= 1;
    }
    bool_vec
}

pub fn random_evals<F: Field>(v: usize) -> Vec<(usize, F)> {
    let mut rng = thread_rng();
    (0..2_usize.pow(v as u32))
        .map(|i| (i, F::rand(&mut rng)))
        .collect()
}
