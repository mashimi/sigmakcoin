use blockchain::{Block, Transaction, TxInput, TxOutput, BlockchainState};

#[test]
fn test_utxo_validation_and_spending() {
    let mut state = BlockchainState::new();
    
    // 1. Initial funding (Genesis setup)
    let initial_tx_id = [1u8; 32];
    state.utxos.insert((initial_tx_id, 0), TxOutput {
        amount: 1000,
        recipient: "alice".to_string(),
    });

    assert_eq!(state.get_balance("alice"), 1000);

    // 2. Create a transaction: Alice spends 400 to Bob, 100 to Charlie, 50 fee
    let tx = Transaction {
        inputs: vec![TxInput {
            tx_id: initial_tx_id,
            output_index: 0,
            signature: vec![], // signature check not implemented in logic yet
        }],
        outputs: vec![
            TxOutput { amount: 400, recipient: "bob".to_string() },
            TxOutput { amount: 100, recipient: "charlie".to_string() },
            TxOutput { amount: 450, recipient: "alice".to_string() }, // Change
        ],
        fee: 50,
        timestamp: 123456789,
    };

    // 3. Apply block containing this transaction
    let mut block = Block::new_genesis();
    block.height = 1;
    block.proposer = "miner".to_string();
    block.transactions = vec![tx];
    
    let result = state.apply_block(&block);
    assert!(result.is_ok(), "Block application failed: {:?}", result.err());

    // 4. Verify balances
    assert_eq!(state.get_balance("bob"), 400);
    assert_eq!(state.get_balance("charlie"), 100);
    assert_eq!(state.get_balance("alice"), 450);
    assert_eq!(state.get_balance("miner"), 5_000_000); // Miner reward
}

#[test]
fn test_double_spend_prevention() {
    let mut state = BlockchainState::new();
    let tx_id = [1u8; 32];
    state.utxos.insert((tx_id, 0), TxOutput {
        amount: 1000,
        recipient: "alice".to_string(),
    });

    let tx = Transaction {
        inputs: vec![TxInput { tx_id, output_index: 0, signature: vec![] }],
        outputs: vec![TxOutput { amount: 1000, recipient: "bob".to_string() }],
        fee: 0,
        timestamp: 1,
    };

    let mut block = Block::new_genesis();
    block.height = 1;
    block.transactions = vec![tx.clone(), tx.clone()]; // Same tx twice in a block

    let result = state.apply_block(&block);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("UTXO not found or already spent"));
}
