use std::collections::HashMap;
use crate::block::{Block, Transaction, TxOutput};

pub struct BlockchainState {
    pub utxos: HashMap<([u8; 32], u32), TxOutput>, // (tx_id, index) -> Output
    pub stakes: HashMap<String, u64>,
}

impl BlockchainState {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
            stakes: HashMap::new(),
        }
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<(), String> {
        for tx in &block.transactions {
            self.apply_transaction(tx)?;
        }
        
        // Coinbase: Handle miner/proposer reward (5 ΣKC)
        let reward_output = TxOutput {
            amount: 5_000_000,
            recipient: block.proposer.clone(),
        };
        
        // Simplified reward: hash of block as tx_id for reward UTXO
        self.utxos.insert((block.calculate_hash(), 0), reward_output);
        
        Ok(())
    }

    fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        let mut input_sum = 0;
        
        // 1. Verify and spend inputs
        for input in &tx.inputs {
            let key = (input.tx_id, input.output_index);
            if let Some(output) = self.utxos.remove(&key) {
                input_sum += output.amount;
            } else {
                return Err("UTXO not found or already spent".to_string());
            }
        }

        // 2. Verify outputs
        let output_sum: u64 = tx.outputs.iter().map(|o| o.amount).sum();
        if input_sum < output_sum + tx.fee {
            return Err("Insufficient input for outputs and fee".to_string());
        }

        // 3. Create new UTXOs
        let tx_id = tx.calculate_id();
        for (i, output) in tx.outputs.iter().enumerate() {
            self.utxos.insert((tx_id, i as u32), output.clone());
        }
        
        Ok(())
    }
    
    pub fn get_balance(&self, address: &str) -> u64 {
        self.utxos.values()
            .filter(|o| o.recipient == address)
            .map(|o| o.amount)
            .sum()
    }
}
