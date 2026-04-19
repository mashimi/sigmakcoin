use axum::{Json, Router, routing::post};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sigmak_zk as verifier;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Transaction {
    sender: String,
    recipient: String,
    amount: u64,
    data_hash: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct BlockHeader {
    parent_hash: String,
    merkle_root: String,
    task_id: String,
    model_hash: String,
    timestamp: u64,
    nonce: u64,
}

impl BlockHeader {
    fn hash(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.parent_hash.as_bytes());
        hasher.update(self.merkle_root.as_bytes());
        hasher.update(self.task_id.as_bytes());
        hasher.update(self.model_hash.as_bytes());
        hasher.update(self.timestamp.to_le_bytes());
        hasher.update(self.nonce.to_le_bytes());
        format!("{:x}", hasher.finalize())
    }
}

#[derive(Deserialize)]
struct GradientProof {
    loss_before: f32,
    loss_after: f32,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct CandidateBlock {
    header: BlockHeader,
    transactions: Vec<Transaction>,
    proof: GradientProof,
}

#[derive(Serialize)]
struct FinalityResponse {
    valid: bool,
    block_hash: Option<String>,
    validator_signatures: Vec<String>,
    message: String,
}

fn decode_hex_32(hex: &str) -> Option<[u8; 32]> {
    if hex.len() != 64 {
        return None;
    }
    let mut bytes = [0u8; 32];
    for i in 0..32 {
        let byte = u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).ok()?;
        bytes[i] = byte;
    }
    Some(bytes)
}

fn convert_candidate(block: &CandidateBlock) -> Option<verifier::Block> {
    let transactions = block
        .transactions
        .iter()
        .map(|tx| {
            Some(verifier::Transaction {
                sender: tx.sender.clone(),
                recipient: tx.recipient.clone(),
                amount: tx.amount,
                data_hash: decode_hex_32(&tx.data_hash)?,
            })
        })
        .collect::<Option<Vec<_>>>()?;

    let parent_hash = decode_hex_32(&block.header.parent_hash)?;
    let merkle_root = decode_hex_32(&block.header.merkle_root)?;
    let model_hash = decode_hex_32(&block.header.model_hash)?;

    let header = verifier::BlockHeader {
        parent_hash,
        merkle_root,
        task_id: block.header.task_id.clone(),
        model_hash,
        timestamp: block.header.timestamp,
        nonce: block.header.nonce,
    };

    let proof = verifier::GradientProof::new(block.proof.loss_before, block.proof.loss_after);

    Some(verifier::Block {
        header,
        transactions,
        proof,
    })
}

fn verify_candidate(block: &CandidateBlock) -> bool {
    if block.header.task_id.is_empty() {
        return false;
    }
    match convert_candidate(block) {
        Some(candidate) => verifier::verify_block(&candidate).unwrap_or(false),
        None => false,
    }
}

fn finalize_candidate(block_hash: &str) -> Vec<String> {
    vec![
        format!("validator-1:sign({})", block_hash),
        format!("validator-2:sign({})", block_hash),
        format!("validator-3:sign({})", block_hash),
    ]
}

async fn submit_block(Json(req): Json<CandidateBlock>) -> Json<FinalityResponse> {
    let valid = verify_candidate(&req);
    let block_hash = if valid {
        Some(req.header.hash())
    } else {
        None
    };
    let signatures = block_hash
        .as_ref()
        .map(|hash| finalize_candidate(hash))
        .unwrap_or_else(Vec::new);
    let message = if valid {
        "Block accepted and finalized by 3 validators".to_string()
    } else {
        "Invalid proof or block candidate".to_string()
    };

    Json(FinalityResponse {
        valid,
        block_hash,
        validator_signatures: signatures,
        message,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/submit", post(submit_block));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Validator running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
