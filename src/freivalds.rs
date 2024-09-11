use crate::ip::*;
use ark_ff::{Fp64, MontBackend, MontConfig, One};
use ark_std::UniformRand;
use ndarray::{Array1, Array2};

#[derive(MontConfig)]
#[modulus = "17"]
#[generator = "2"]
struct FrConfig;

type F = Fp64<MontBackend<FrConfig, 1>>;

// IP to verify that A*B = C for matrices A, B, and C
#[derive(Debug, Clone)]
struct MatMulIP {
    status: Status,
    a_array: Array2<F>,
    b_array: Array2<F>,
    c_array: Option<Array2<F>>,
    prover_mode: ProverMode,
}

impl IP for MatMulIP {
    type Message = Option<Array2<F>>;
    type Input = (Array2<F>, Array2<F>);

    fn initialize(input: Self::Input, prover_mode: ProverMode) -> Self {
        Self {
            c_array: None,
            status: Status::Running,
            a_array: input.0,
            b_array: input.1,
            prover_mode,
        }
    }

    fn run_honest_prover_logic(&mut self) -> Self::Message {
        let c_array = self.a_array.dot(&self.b_array);
        Some(c_array)
    }

    fn run_malicious_prover_logic(&mut self) -> Self::Message {
        let mut c_array = self.a_array.dot(&self.b_array);
        // Manipulate one of the elements
        c_array[(0, 0)] += F::one();
        Some(c_array)
    }

    fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn run_verifier_logic(&mut self) -> Self::Message {
        let c_array = self.get_last_message().unwrap();
        let mut rng = rand::thread_rng();
        let mut r_powers = Array1::<F>::default(c_array.dim().0);

        let r = F::rand(&mut rng);
        let mut r_power = F::one();
        for power in r_powers.iter_mut() {
            *power = r_power;
            r_power *= r;
        }

        let c_prod = c_array.dot(&r_powers);
        let a_b_prod = self.a_array.dot(&self.b_array.dot(&r_powers));

        if c_prod == a_b_prod {
            self.status = Status::Accepted;
        } else {
            self.status = Status::Rejected;
        }

        None
    }

    fn add_message(&mut self, message: Self::Message) {
        self.c_array = message;
    }

    fn total_messages(&self) -> usize {
        self.c_array.as_ref().map_or(0, |_| 1)
    }

    fn get_last_message(&self) -> Self::Message {
        self.c_array.clone()
    }

    fn get_prover_mode(&self) -> ProverMode {
        self.prover_mode.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frievalds_ip() {
        let n = 20;
        let a = Array2::<F>::from_elem((n, n), F::one());
        let b = Array2::<F>::from_elem((n, n), F::one() + F::one());

        let mut honest_frievalds_ip =
            MatMulIP::initialize((a.clone(), b.clone()), ProverMode::Honest);
        let _ = honest_frievalds_ip.step();
        let _ = honest_frievalds_ip.step();
        assert_eq!(honest_frievalds_ip.get_status(), Status::Accepted);

        let mut malicious_sum_ip = MatMulIP::initialize((a, b), ProverMode::Malicious);
        let _ = malicious_sum_ip.step();
        let _ = malicious_sum_ip.step();
        assert_eq!(malicious_sum_ip.get_status(), Status::Rejected);
    }
}
