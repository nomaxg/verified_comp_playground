#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Running,
    Accepted,
    Rejected,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ProverMode {
    Honest,
    Malicious,
}

// TODO use a macro to make some base IP that pushes to a queue of messages
pub trait IP {
    type ProverMessage;
    type VerifierMessage;
    type Input;

    fn initialize(input: Self::Input, prover_mode: ProverMode) -> Self;
    fn add_verifier_message(&mut self, message: Self::VerifierMessage);
    fn add_prover_message(&mut self, message: Self::ProverMessage);
    fn get_last_prover_message(&self) -> Self::ProverMessage;
    fn get_last_verifier_message(&self) -> Self::VerifierMessage;
    fn total_messages(&self) -> usize;
    fn run_honest_prover_logic(&mut self) -> Self::ProverMessage;
    fn run_malicious_prover_logic(&mut self) -> Self::ProverMessage;
    fn run_verifier_logic(&mut self) -> Self::VerifierMessage;
    fn get_status(&self) -> Status;
    fn get_prover_mode(&self) -> ProverMode;

    fn step(&mut self) -> Status {
        let status = self.get_status();
        // If decided, return
        if status != Status::Running {
            return status;
        }

        let num_messages = self.total_messages();

        if num_messages % 2 == 0 {
            let prover_message = match self.get_prover_mode() {
                ProverMode::Honest => self.run_honest_prover_logic(),
                ProverMode::Malicious => self.run_malicious_prover_logic(),
            };
            self.add_prover_message(prover_message);
        } else {
            let verifier_message = self.run_verifier_logic();
            self.add_verifier_message(verifier_message);
        }

        self.get_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Super simple IP where the prover accepts if the verifier can submit two numbers that sum to
    // a given target
    #[derive(Debug, Clone)]
    struct SumIP {
        target_sum: u64,
        messages: Vec<(u64, u64)>,
        status: Status,
        prover_mode: ProverMode,
    }

    impl IP for SumIP {
        type ProverMessage = (u64, u64);
        type VerifierMessage = ();
        type Input = u64;

        fn initialize(input: Self::Input, prover_mode: ProverMode) -> Self {
            Self {
                target_sum: input,
                messages: vec![],
                status: Status::Running,
                prover_mode,
            }
        }

        fn run_honest_prover_logic(&mut self) -> Self::ProverMessage {
            (1, self.target_sum - 1)
        }

        fn run_malicious_prover_logic(&mut self) -> Self::ProverMessage {
            (2, self.target_sum - 1)
        }

        fn get_status(&self) -> Status {
            self.status.clone()
        }

        fn run_verifier_logic(&mut self) -> Self::VerifierMessage {
            let last_message = self.get_last_prover_message();
            if last_message.0 + last_message.1 == self.target_sum {
                self.status = Status::Accepted;
            } else {
                self.status = Status::Rejected;
            }
        }

        fn add_prover_message(&mut self, message: Self::ProverMessage) {
            self.messages.push(message);
        }

        fn add_verifier_message(&mut self, message: Self::VerifierMessage) {}

        fn total_messages(&self) -> usize {
            self.messages.len()
        }

        fn get_last_prover_message(&self) -> Self::ProverMessage {
            *self.messages.last().unwrap()
        }

        fn get_last_verifier_message(&self) -> Self::VerifierMessage {}

        fn get_prover_mode(&self) -> ProverMode {
            self.prover_mode.clone()
        }
    }

    #[test]
    fn test_sum_ip() {
        let mut honest_sum_ip = SumIP::initialize(100, ProverMode::Honest);
        let _ = honest_sum_ip.step();
        let _ = honest_sum_ip.step();
        assert_eq!(honest_sum_ip.get_status(), Status::Accepted);

        let mut malicious_sum_ip = SumIP::initialize(100, ProverMode::Malicious);
        let _ = malicious_sum_ip.step();
        let _ = malicious_sum_ip.step();
        assert_eq!(malicious_sum_ip.get_status(), Status::Rejected);
    }
}
