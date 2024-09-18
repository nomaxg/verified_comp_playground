use crate::ip::*;
use ark_ff::Field;

// IP representing the sum check protocol
#[derive(Debug, Clone)]
struct SumCheck<F: Field> {
    status: Status,
    prover_mode: ProverMode,
    r: Vec<F>,
    univariate_evals: Vec<Vec<F>>,
}

// Verifier sends random values, verifier sends values representing full evaluations of the
// univariate polynomial g_i over field F.
//
// TODO: modify to send (0,...,deg_(g)) evaluations once we start using bigger fields.
enum Message<F: Field> {
    VerifierMessage(F),
    ProverMessage(Vec<F>),
}

// impl<F> IP for SumCheck<F>
// where
//     F: Field,
// {
//     type Message = Message<F>;
//
//     // Input is just v, the number of variables for some random set of evaluations of the boolean
//     // hypercube over a multilinear polynomial.
//     //
//     // TODO: make this less contrived and actually pass in the polynomial.
//     type Input = usize;
//
//     fn initialize(input: Self::Input, prover_mode: ProverMode) -> Self {
//         Self {
//             status: Status::Running,
//             prover_mode,
//             r: vec![],
//             univariate_evals: vec![],
//         }
//     }
//
//     fn run_honest_prover_logic(&mut self) -> Self::Message {
//         let c_array = self.a_array.dot(&self.b_array);
//         Some(c_array)
//     }
//
//     fn run_malicious_prover_logic(&mut self) -> Self::Message {
//         let mut c_array = self.a_array.dot(&self.b_array);
//         // Manipulate one of the elements
//         c_array[(0, 0)] += F::one();
//         Some(c_array)
//     }
//
//     fn get_status(&self) -> Status {
//         self.status.clone()
//     }
//
//     fn run_verifier_logic(&mut self) -> Self::Message {
//         let c_array = self.get_last_message().unwrap();
//         let mut rng = rand::thread_rng();
//         let mut r_powers = Array1::<F>::default(c_array.dim().0);
//
//         let r = F::rand(&mut rng);
//         let mut r_power = F::one();
//         for power in r_powers.iter_mut() {
//             *power = r_power;
//             r_power *= r;
//         }
//
//         let c_prod = c_array.dot(&r_powers);
//         let a_b_prod = self.a_array.dot(&self.b_array.dot(&r_powers));
//
//         if c_prod == a_b_prod {
//             self.status = Status::Accepted;
//         } else {
//             self.status = Status::Rejected;
//         }
//
//         None
//     }
//
//     fn add_message(&mut self, message: Self::Message) {
//         self.c_array = message;
//     }
//
//     fn total_messages(&self) -> usize {
//         self.c_array.as_ref().map_or(0, |_| 1)
//     }
//
//     fn get_last_message(&self) -> Self::Message {
//         self.c_array.clone()
//     }
//
//     fn get_prover_mode(&self) -> ProverMode {
//         self.prover_mode.clone()
//     }
// }
