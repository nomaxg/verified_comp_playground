use crate::{
    fields::random_elem,
    ip::*,
    mle::{calculate_g_i, random_evals},
};
use ark_ff::Field;
use ark_std::iterable::Iterable;

// IP representing the sum check protocol
#[derive(Debug, Clone)]
struct SumCheck<F: Field> {
    status: Status,
    prover_mode: ProverMode,
    r: Vec<F>,
    univariate_evals: Vec<Vec<F>>,
    hypercube_evals: Vec<F>,
    v: usize,
}

impl<F> IP for SumCheck<F>
where
    F: Field,
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
        Self {
            status: Status::Running,
            prover_mode,
            r: vec![],
            univariate_evals: vec![],
            // TODO: make this less contrived and actually pass in the polynomial.
            hypercube_evals: random_evals(input),
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
                                  // affect the sum check in the next round
        g_i_evals
    }

    fn get_status(&self) -> Status {
        self.status.clone()
    }

    fn run_verifier_logic(&mut self) -> Self::VerifierMessage {
        random_elem::<F>()
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
