use blockchain::{Block, Transaction};
use consensus::{DPoSEngine, Validator};
use proof_verifier::generate_gradient_proof;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_block_validation_success() {
    let mut engine = DPoSEngine::new(1000, 10);
    engine.validators.push(Validator {
        address: "miner_1".to_string(),
        stake: 5000,
        reputation: 100,
    });

    let genesis = Block::new_genesis();
    
    // Create a valid proof (Loss 2.5 -> 2.4 = 0.1 reduction, which is 10 in micro-units)
    let proof = generate_gradient_proof(2.5, 2.4, 0.5, 50);

    let next_block = Block {
        height: 1,
        timestamp: genesis.timestamp + 10,
        prev_hash: genesis.calculate_hash(),
        transactions: vec![],
        proposer: "miner_1".to_string(),
        proof_of_intelligence: proof,
        nonce: 0,
    };

    let result = engine.validate_block(&next_block, &genesis);
    assert!(result.is_ok(), "Validation failed: {:?}", result.err());
}

#[test]
fn test_block_validation_invalid_proof() {
    let mut engine = DPoSEngine::new(1000, 100); // Higher threshold
    engine.validators.push(Validator {
        address: "miner_1".to_string(),
        stake: 5000,
        reputation: 100,
    });

    let genesis = Block::new_genesis();
    
    // Proof with low reduction
    let proof = generate_gradient_proof(2.5, 2.45, 0.5, 50);

    let next_block = Block {
        height: 1,
        timestamp: genesis.timestamp + 10,
        prev_hash: genesis.calculate_hash(),
        transactions: vec![],
        proposer: "miner_1".to_string(),
        proof_of_intelligence: proof,
        nonce: 0,
    };

    let result = engine.validate_block(&next_block, &genesis);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("loss reduction threshold not met"));
}
