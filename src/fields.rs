use std::usize;

use ark_ff::{Field, Fp64, MontBackend, MontConfig};
use rand::thread_rng;

#[derive(MontConfig)]
#[modulus = "5"]
#[generator = "2"]
pub struct FrConfig;

pub type Fr = Fp64<MontBackend<FrConfig, 1>>;

pub fn random_elem<F: Field>() -> F {
    let mut rng = thread_rng();
    F::rand(&mut rng)
}

pub fn random_vec<F: Field>(v: usize) -> Vec<F> {
    (0..v).map(|_| random_elem()).collect()
}

pub fn bool_to_field<F: Field>(b: bool) -> F {
    if b {
        F::one()
    } else {
        F::zero()
    }
}

pub fn evals_sum<F: Field>(evals: &[F]) -> F {
    evals.iter().fold(F::zero(), |acc, &x| acc + x)
}
