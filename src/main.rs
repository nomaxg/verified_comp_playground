use ark_ff::{Field, Fp64, MontBackend, MontConfig, PrimeField};
use ark_std::{rand::Rng, UniformRand};

#[derive(MontConfig)]
#[modulus = "17"]
#[generator = "2"]
struct FrConfig;

type Fr = Fp64<MontBackend<FrConfig, 1>>;

#[derive(Debug, Clone)]
struct HashFn {
    eval_point: Fr,
}

impl HashFn {
    pub fn new<R: Rng>(rng: &mut R) -> Self {
        // Generate a random field element
        let random_element: Fr = UniformRand::rand(rng);
        Self {
            eval_point: random_element,
        }
    }

    pub fn new_from_eval(eval_point: Fr) -> Self {
        Self { eval_point }
    }

    pub fn hash(&self, evals: Vec<Fr>) -> Fr {
        let mut res = Fr::from(0);
        for (i, eval) in evals.iter().enumerate() {
            res += self.eval_point.pow([i as u64]) * eval;
        }
        res
    }

    pub fn get_eval_point(&self) -> Fr {
        self.eval_point
    }
}

fn run_reed_solomon_communication_protocol(bob_file: Vec<u64>, alice_file: Vec<u64>) {
    let mut rng = ark_std::test_rng();

    // Alice and Bob have a file of length n
    assert!(bob_file.len() == alice_file.len());

    let alice_evals: Vec<_> = alice_file
        .iter()
        .map(|f| Fr::from_bigint((*f).into()).unwrap())
        .collect();
    let bob_evals: Vec<_> = bob_file
        .iter()
        .map(|f| Fr::from_bigint((*f).into()).unwrap())
        .collect();

    // Alice will generate a hash function from the family Hr
    let alice_hash = HashFn::new(&mut rng);

    // Alice generates her fingerprint
    let alice_fingerprint = alice_hash.hash(alice_evals);

    // Alice can extract the evaluation point that parameterizes her hash fn for Bob
    let alice_eval_point = alice_hash.get_eval_point();

    // Bob can construct the hash function, hash on his own, and assert that his fingerprint
    // equals Alice's fingerprint
    let bob_hash = HashFn::new_from_eval(alice_eval_point);
    let bob_fingerprint = bob_hash.hash(bob_evals);
    assert!(bob_fingerprint == alice_fingerprint);
}

fn main() {
    let alice_file = vec![1, 2, 3, 4];
    let bob_file = vec![1, 2, 3, 4];
    run_reed_solomon_communication_protocol(alice_file, bob_file)
}
