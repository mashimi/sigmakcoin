use std::collections::HashMap;
use crate::block::{Block, Transaction};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct UTXO {
    pub tx_id: [u8; 32],
    pub output_index: u32,
    pub amount: u64,
    pub owner: String,
}

pub struct BlockchainState {
    pub utxos: HashMap<String, Vec<UTXO>>, // owner -> UTXOs
    pub balances: HashMap<String, u64>,
    pub stakes: HashMap<String, u64>,
}

impl BlockchainState {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
            balances: HashMap::new(),
            stakes: HashMap::new(),
        }
    }

    pub fn apply_block(&mut self, block: &Block) -> Result<(), String> {
        for tx in &block.transactions {
            self.apply_transaction(tx)?;
        }
        
        // Handle miner/proposer distribution (simplified)
        let reward = 5_000_000; // 5 ΣKC
        *self.balances.entry(block.proposer.clone()).or_insert(0) += reward;
        
        Ok(())
    }

    fn apply_transaction(&mut self, tx: &Transaction) -> Result<(), String> {
        // Simple balance check (ignoring UTXO logic for this PoC to keep it simple)
        let sender_balance = self.balances.get(&tx.from).cloned().unwrap_or(0);
        if sender_balance < tx.amount + tx.fee {
            return Err(format!("Insufficient balance for {}", tx.from));
        }

        *self.balances.entry(tx.from.clone()).or_insert(0) -= tx.amount + tx.fee;
        *self.balances.entry(tx.to.clone()).or_insert(0) += tx.amount;
        
        Ok(())
    }
}
