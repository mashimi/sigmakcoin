use anyhow::Result;
use blockchain::{Block, BlockchainState};
use consensus::DPoSEngine;
use ipfs_storage;
use p2p_network;

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔════════════════════════════════════════╗");
    println!("║     ΣKCoin Node Starting...           ║");
    println!("╚════════════════════════════════════════╝");

    // Initialize Blockchain State with Genesis Block
    let mut state = BlockchainState::new();
    let genesis = Block::new_genesis();
    println!("✨ Genesis block initialized: {}", hex::encode(genesis.calculate_hash()));

    // Apply Genesis distribution (Seed some balances)
    state.balances.insert("sig1genesis_distributor".to_string(), 100_000_000); // 100 ΣKC
    state.apply_block(&genesis).map_err(|e| anyhow::anyhow!(e))?;

    // Initialize Consensus Engine
    let mut consensus = DPoSEngine::new();
    consensus.validators.push(consensus::Validator {
        address: "sig1genesis_distributor".to_string(),
        stake: 10_000,
        reputation: 100,
    });

    println!("🏛️  Blockchain state ready. Initial balance: {} ΣKC", 
        state.balances.get("sig1genesis_distributor").unwrap_or(&0) / 1_000_000);

    // Start Services
    let storage_handle = tokio::spawn(async {
        ipfs_storage::run().await
    });

    let network_handle = tokio::spawn(async {
        p2p_network::run().await
    });

    let verifier_handle = tokio::spawn(async {
        println!("🔐 ZK Verifier service active");
        // In a real node, this would listen for new proofs to verify
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        Ok::<(), anyhow::Error>(())
    });

    tokio::try_join!(storage_handle, network_handle, verifier_handle)?;

    Ok(())
}
