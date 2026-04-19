use anyhow::Result;
use p2p_network::SigmaKNetwork;
use libp2p::identity;

#[tokio::main]
async fn main() -> Result<()> {
    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = libp2p::PeerId::from(local_key.public());

println!("🚀 Starting ΣKCoin Node: {}", peer_id);

    let mut network = SigmaKNetwork::new(peer_id, local_key).await.map_err(|e| {
        anyhow::Error::new(e)
    })?;

    // Broadcast a test proof
    let test_proof = p2p_network::NetworkMessage::NewProof {
        miner_id: peer_id.to_string(),
        proof_hash: "abc123".to_string(),
        model_cid: "QmTest".to_string(),
        loss_reduction: 0.42,
    };

    network.broadcast_proof(test_proof).map_err(|e| {
        anyhow::Error::new(e)
    })?;

    network.run().await.map_err(|e| {
        anyhow::Error::new(e)
    })?;

    Ok(())
}
