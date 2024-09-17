use crate::fields::{bool_to_field, random_elem};
use ark_ff::Field;
use ark_std::iterable::Iterable;
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

pub fn random_evals<F: Field>(v: usize) -> Vec<F> {
    let mut rng = thread_rng();
    (0..2_usize.pow(v as u32))
        .map(|_| F::rand(&mut rng))
        .collect()
}

pub fn w_basis_eval<F: Field>(r: &[F], eval: (usize, F), v: usize) -> F {
    let bools = index_to_bools(eval.0, v);
    let mut prod = F::one();

    for i in 0..v {
        let bool_field_elem = bool_to_field::<F>(bools[i]);
        prod *= r[i] * bool_field_elem + (F::one() - bool_field_elem) * (F::one() - r[i]);
    }

    prod
}

pub fn stream_eval<F: Field>(r: &[F], evals: &Vec<F>, v: usize) -> F {
    let mut res = F::zero();

    for (i, eval) in evals.iter().enumerate() {
        res += evals[i] * w_basis_eval(r, (i, *eval), v);
    }

    res
}

pub fn g_poly<F: Field>(input: &[F]) -> F {
    // Hardcoding polynomial for now
    assert!(input.len() == 3);

    // Use example in book 2X1^3 + X1X3 + X2X3
    F::from(2u64) * input[0].pow([3u64]) + input[1] * input[2] + input[2] * input[3]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fr;

    #[test]
    fn test() {
        let v = 2;
        let evals = vec![Fr::from(1), Fr::from(1), Fr::from(2), Fr::from(4)];

        for i in 0..5 {
            for j in 0..5 {
                let r = vec![Fr::from(i), Fr::from(j)];
                let res = stream_eval::<Fr>(&r, &evals, v);
                print!("{} ", res);
            }
            println!();
        }
    }
}
