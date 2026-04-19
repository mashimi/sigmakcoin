use blockchain::Block;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub reputation: u32,
}

pub struct DPoSEngine {
    pub validators: Vec<Validator>,
}

impl DPoSEngine {
    pub fn new() -> Self {
        Self {
            validators: vec![],
        }
    }

    pub fn select_proposer(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        // Select proposer based on weighted stake
        self.validators
            .choose_weighted(&mut rng, |v| v.stake)
            .map(|v| v.address.clone())
            .ok()
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        // 1. Verify height and timestamp are sensible
        // 2. Proposer must be an active validator
        let is_validator = self.validators.iter().any(|v| v.address == block.proposer);
        
        // 3. ZK Proof validation (would call proof_verifier here)
        
        is_validator
    }
}
