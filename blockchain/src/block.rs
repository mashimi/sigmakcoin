use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub fee: u64,
    pub proof_cid: Option<String>, // Reference to IPFS stored proof
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Block {
    pub height: u64,
    pub timestamp: u64,
    pub prev_hash: [u8; 32],
    pub transactions: Vec<Transaction>,
    pub proposer: String,
    pub proof_of_intelligence: Vec<u8>, // ZK proof bytes
    pub nonce: u64,
}

impl Block {
    pub fn calculate_hash(&self) -> [u8; 32] {
        let serialized = serde_json::to_vec(self).expect("Failed to serialize block");
        let hash = Sha256::digest(&serialized);
        hash.into()
    }

    pub fn new_genesis() -> Self {
        Block {
            height: 0,
            timestamp: 1713532800, // Fixed time for genesis
            prev_hash: [0; 32],
            transactions: vec![],
            proposer: "genesis".to_string(),
            proof_of_intelligence: vec![],
            nonce: 0,
        }
    }
}
