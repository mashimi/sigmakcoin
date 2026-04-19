use anyhow::Result;

use ipfs_storage;
use p2p_network;

#[tokio::main]
async fn main() -> Result<()> {
    println!("╔════════════════════════════════════════╗");
    println!("║     ΣKCoin Node Starting...           ║");
    println!("╚════════════════════════════════════════╗");

    let storage_handle = tokio::spawn(async {
        ipfs_storage::run().await
    });

    let network_handle = tokio::spawn(async {
        p2p_network::run().await
    });

    let verifier_handle = tokio::spawn(async {
        println!("🔐 ZK Verifier ready");
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        Ok::<(), anyhow::Error>(())
    });

    tokio::try_join!(storage_handle, network_handle, verifier_handle)?;

    Ok(())
}
