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

    // Apply Genesis distribution (Seed some balances manually for UTXO model)
    state.utxos.insert(([0; 32], 0), blockchain::TxOutput {
        amount: 100_000_000,
        recipient: "sig1genesis_distributor".to_string(),
    });
    
    state.apply_block(&genesis).map_err(|e| anyhow::anyhow!(e))?;

    // Initialize Consensus Engine (Min stake: 10k, Loss threshold: 10)
    let mut consensus = DPoSEngine::new(10_000, 10);
    consensus.validators.push(consensus::Validator {
        address: "sig1genesis_distributor".to_string(),
        stake: 10_000,
        reputation: 100,
    });

    println!("🏛️  Blockchain state ready. Initial balance: {} ΣKC", 
        state.get_balance("sig1genesis_distributor") / 1_000_000);

    // Setup P2P Message Handling
    let (msg_tx, mut msg_rx) = tokio::sync::mpsc::unbounded_channel::<p2p_network::NetworkMessage>();

    // Start Services
    let storage_handle = tokio::spawn(async {
        ipfs_storage::run().await
    });

    let network_handle = tokio::spawn(async move {
        p2p_network::run(msg_tx).await
    });

    let msg_handler_handle = tokio::spawn(async move {
        while let Some(msg) = msg_rx.recv().await {
            match msg {
                p2p_network::NetworkMessage::BlockRequest { start_height, .. } => {
                    println!("📬 Peer requested blocks starting at height {}", start_height);
                }
                p2p_network::NetworkMessage::BlockResponse { blocks } => {
                    println!("📥 Received {} blocks for sync", blocks.len());
                }
                _ => {}
            }
        }
        Ok::<(), anyhow::Error>(())
    });

    let verifier_handle = tokio::spawn(async {
        println!("🔐 ZK Verifier service active");
        // In a real node, this would listen for new proofs to verify
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        Ok::<(), anyhow::Error>(())
    });

    tokio::try_join!(storage_handle, network_handle, verifier_handle, msg_handler_handle)?;

    Ok(())
}
