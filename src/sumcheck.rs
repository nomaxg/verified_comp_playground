use crate::{
    fields::{evals_sum, random_elem},
    ip::*,
    mle::{calculate_g_i, random_evals, stream_eval},
    polynomials::eval_poly,
};
use ark_ff::{FftField, Field};
use ark_std::iterable::Iterable;

// IP representing the sum check protocol
#[derive(Debug, Clone)]
struct SumCheck<F: FftField> {
    status: Status,
    prover_mode: ProverMode,
    r: Vec<F>,
    univariate_evals: Vec<Vec<F>>,
    hypercube_evals: Vec<F>,
    g_sum: F,
    v: usize,
}

impl<F> IP for SumCheck<F>
where
    F: FftField,
{
    // Verifier sends random values, verifier sends values representing full evaluations of the
    // univariate polynomial g_i over field F.
    //
    // TODO: modify to send (0,...,deg_(g)) evaluations once we start using bigger fields.
    type ProverMessage = Vec<F>;
    type VerifierMessage = F;

    // Input is just v, the number of variables for some random set of evaluations of the boolean
    // hypercube over a multilinear polynomial.
    type Input = usize;

    fn initialize(input: Self::Input, prover_mode: ProverMode) -> Self {
        let evals = random_evals(input);
        let sum = evals_sum(&evals);
        Self {
            status: Status::Running,
            prover_mode,
            r: vec![],
            univariate_evals: vec![],
            // TODO: make this less contrived and actually pass in the polynomial.
            hypercube_evals: evals,
            g_sum: sum,
            v: input,
        }
    }

    fn run_honest_prover_logic(&mut self) -> Self::ProverMessage {
        // Assert that v = log_2(len(evals))
        let g_i_evals = calculate_g_i(&self.r, &self.hypercube_evals, self.v);
        g_i_evals
    }

    fn run_malicious_prover_logic(&mut self) -> Self::ProverMessage {
        // Assert that v = log_2(len(evals))
        let mut g_i_evals = calculate_g_i(&self.r, &self.hypercube_evals, self.v);
        g_i_evals[0] += F::one(); // tamper with the univariate polynomial at xi =0, which will
                                  // affect the sum check
        g_i_evals
    }

    fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn run_verifier_logic(&mut self) -> Self::VerifierMessage {
        let univariate_evals = self.get_last_prover_message();
        let round_num = self.r.len();
        let univariate_sum = univariate_evals[0] + univariate_evals[1];
        let rand_response = random_elem::<F>();

        if round_num == 0 {
            // In the first round, verify that the univariate polynomial sums to the hypercube eval sum
            let sum_check_success = univariate_sum == self.g_sum;
            if !sum_check_success {
                dbg!("failing in first round");
                self.status = Status::Rejected;
            }
        } else if round_num < self.v - 1 {
            let previous_poly = &self.univariate_evals[round_num - 1];
            let previous_r = self.r[round_num - 1];
            // For intermediate rounds, verify that the current univariate polynomial sums to the random
            // evaluation of the last univariate polynomial
            let sum_check_success = univariate_sum == eval_poly(previous_poly, previous_r);
            if !sum_check_success {
                self.status = Status::Rejected;
            }
        } else {
            let last_r = [self.r.as_slice(), &[rand_response]].concat();
            let sum_check_pass = eval_poly(&univarate_evals, rand_response)
                == stream_eval(&last_r, &self.hypercube_evals, self.v);
            if !sum_check_pass {
                self.status = Status::Rejected;
            } else {
                self.status = Status::Accepted;
            }
        }
        rand_response
    }

    fn add_prover_message(&mut self, message: Self::ProverMessage) {
        self.univariate_evals.push(message)
    }

    fn add_verifier_message(&mut self, message: Self::VerifierMessage) {
        self.r.push(message)
    }

    fn total_messages(&self) -> usize {
        self.r.len() + self.univariate_evals.len()
    }

    fn get_last_prover_message(&self) -> Self::ProverMessage {
        self.univariate_evals[self.univariate_evals.len() - 1].clone()
    }

    fn get_last_verifier_message(&self) -> Self::VerifierMessage {
        self.r[self.r.len()]
    }

    fn get_prover_mode(&self) -> ProverMode {
        self.prover_mode.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fields::Fr;

    #[test]
    fn test_sumcheck_ip() {
        let v = 8;

        let mut honest_sumcheck_ip: SumCheck<Fr> = SumCheck::initialize(v, ProverMode::Honest);
        for _ in 0..v * 2 {
            let _ = honest_sumcheck_ip.step();
        }
        assert_eq!(honest_sumcheck_ip.get_status(), Status::Accepted);
    }
}
