use blockchain::Block;
use sigmak_zk::verify_gradient_proof;
use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Validator {
    pub address: String,
    pub stake: u64,
    pub reputation: u32,
}

pub struct DPoSEngine {
    pub validators: Vec<Validator>,
    pub min_stake: u64,
    pub loss_threshold: u32,
}

impl DPoSEngine {
    pub fn new(min_stake: u64, loss_threshold: u32) -> Self {
        Self {
            validators: vec![],
            min_stake,
            loss_threshold,
        }
    }

    pub fn select_proposer(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        // Weighted selection by stake
        self.validators
            .choose_weighted(&mut rng, |v| v.stake)
            .map(|v| v.address.clone())
            .ok()
    }

    pub fn validate_block(&self, block: &Block, last_block: &Block) -> Result<bool, String> {
        // 1. Height and Hash linkage
        if block.height != last_block.height + 1 {
            return Err(format!("Invalid block height: expected {}, got {}", last_block.height + 1, block.height));
        }
        if block.prev_hash != last_block.calculate_hash() {
            return Err("Previous block hash mismatch".to_string());
        }

        // 2. Timestamp sanity
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        if block.timestamp > now + 60 {
            return Err("Block timestamp is too far in the future".to_string());
        }
        if block.timestamp <= last_block.timestamp {
            return Err("Block timestamp must be after previous block".to_string());
        }

        // 3. Proposer selection validation
        let validator = self.validators.iter().find(|v| v.address == block.proposer)
            .ok_or_else(|| "Block proposer is not in the active validator set".to_string())?;
        
        if validator.stake < self.min_stake {
            return Err(format!("Proposer stake {} is below minimum {}", validator.stake, self.min_stake));
        }

        // 4. ZK-Proof of Intelligence Work
        if !block.proof_of_intelligence.is_empty() {
            let is_valid_proof = verify_gradient_proof(&block.proof_of_intelligence, self.loss_threshold)
                .map_err(|e| format!("ZK proof verification error: {}", e))?;
            
            if !is_valid_proof {
                return Err("Invalid Proof of Intelligence Work: loss reduction threshold not met".to_string());
            }
        } else if block.height > 0 {
            return Err("Missing required ZK proof for non-genesis block".to_string());
        }

        Ok(true)
    }
}
