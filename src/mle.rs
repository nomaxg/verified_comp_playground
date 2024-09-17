use crate::fields::bool_to_field;
use ark_ff::Field;
use ark_std::iterable::Iterable;
use rand::thread_rng;

// We need 2^v evaluation points

pub fn index_to_vars<F: Field>(mut index: usize, v: usize) -> Vec<F> {
    let mut vars_vec = vec![F::zero(); v];
    let mut place = 0;
    assert!((index as f64).log2() < (v as f64));
    while index > 0 {
        vars_vec[place] = bool_to_field(index & 1 == 1);
        place += 1;
        index >>= 1;
    }
    vars_vec
}

pub fn random_evals<F: Field>(v: usize) -> Vec<F> {
    let mut rng = thread_rng();
    (0..2_usize.pow(v as u32))
        .map(|_| F::rand(&mut rng))
        .collect()
}

pub fn w_basis_eval<F: Field>(r: &[F], eval: (usize, F), v: usize) -> F {
    let vars = index_to_vars::<F>(eval.0, v);
    let mut prod = F::one();

    for i in 0..v {
        prod *= r[i] * vars[i] + (F::one() - vars[i]) * (F::one() - r[i]);
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
    F::from(2u64) * input[0].pow([3u64]) + input[1] * input[2] + input[1] * input[2]
}

// Given a list of evals, calculate the univariate polynomial for variable xi
// With x1,..xi-1 fixed with random values and xi+1 summed away.
//
// Returns |F| evaluations of the resulting univariate polynomial
pub fn calculate_g_i<F: Field>(randoms: &[F], evals: &[F], v: usize) -> Vec<F> {
    // TODO: hardcoded field  field size, find some way to fix this
    for i in 0..5 {
        let mut partial_sum = F::zero();
        for index in 0..v - randoms.len() - 1 {
            let vars = index_to_vars(index, v);
            let r = (randoms.concat(&[F::from(i)])).concat(vars);
            partial_sum += stream_eval(r, evals, v)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fr;

    #[test]
    fn test_g_sum() {
        let mut sum = Fr::from(0);
        for i in 0..8 {
            let vars = index_to_vars::<Fr>(i, 3);
            sum += g_poly(&vars);
        }
        assert!(sum == Fr::from(12))
    }

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
