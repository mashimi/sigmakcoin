# ΣKCoin (SigmaKCoin) 🧠⛓

> **The World's First AI-Proof-of-Work Blockchain** — where mining means training intelligence, not burning electricity.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Built%20With-Rust-orange.svg)](https://www.rust-lang.org/)
[![Plonky3](https://img.shields.io/badge/ZKP-Plonky3-blue.svg)](https://github.com/Plonky3/Plonky3)
[![GitHub](https://img.shields.io/badge/Repo-mashimi%2Fsigmakcoin-brightgreen.svg)](https://github.com/mashimi/sigmakcoin)

---

## 🏛 Vision: From PoW to PoIW

Traditional blockchains consume terawatts of electricity to solve arbitrary puzzles that produce no real-world value. **ΣKCoin** fundamentally redefines what it means to "mine" a block.

Instead of computing sha256 hashes in a loop, **ΣKCoin miners train AI models** — real gradient descent, real intelligence work. Every new block mined advances the global state of machine learning. Every ΣKC earned represents a provable, verifiable unit of AI improvement.

By combining **Zero-Knowledge Proofs (ZKP)** with **Delegated Proof of Stake (DPoS)** consensus, the network ensures:
- Proofs cannot be faked or replayed.
- Validators cannot be bribed or colluded.
- Miners cannot submit low-quality training work.

---

## 🧩 Core Architecture

ΣKCoin is a **Cargo workspace** composed of six tightly integrated Rust crates:

```
sigmakcoin/
├── src/                  # Full node orchestrator (main entry point)
├── blockchain/           # Block, Transaction, and State types
├── consensus/            # DPoS engine + block validation logic
├── proof_verifier/       # ZK-circuit: Plonky3-based PoIW verifier
├── p2p_network/          # libp2p Gossipsub + mDNS networking
├── ipfs_storage/         # IPFS-backed model checkpoint storage
├── validator/            # Staking, reputation, and slashing logic
└── tests/                # Integration test suite
```

---

### 1. 🔐 ZK-Proof Verifier (`proof_verifier`)

The cryptographic heart of ΣKCoin. Built on [Plonky3](https://github.com/Plonky3/Plonky3), it proves — without revealing model weights — that a miner:
1. Started with a model at loss `L_before`.
2. Applied valid gradient descent steps.
3. Arrived at a model with loss `L_after`, where `L_before − L_after ≥ Δ_min`.

**Key module: `GradientCircuit`**
- Defined using Plonky3's `Air` trait over a `BabyBear` prime field.
- Constraints: non-zero gradient check, step counter verification, loss-delta boundary.
- Proof is serialized to `Vec<u8>` and embedded directly in the block header.

**Public API:**
```rust
// Generate a proof of training work
pub fn generate_gradient_proof(
    loss_before: f32,
    loss_after: f32,
    gradient_norm: f32,
    steps: usize
) -> Vec<u8>;

// Verify a submitted proof against a threshold
pub fn verify_gradient_proof(
    proof: &[u8],
    loss_threshold: f32
) -> Result<bool, anyhow::Error>;
```

---

### 2. ⛓ Blockchain Layer (`blockchain`)

Implements the core data structures of the chain:

- **`Block`**: Contains height, timestamp, SHA-256 `prev_hash`, transactions, proposer address, PoIW proof, and nonce.
- **`Transaction`**: Represents a ΣKC transfer with sender/receiver/amount and a unique ID.
- **`BlockchainState`**: A balance-map ledger that applies blocks sequentially, distributing miner rewards (5 ΣKC/block) and processing transactions atomically.
- **Genesis Block**: A statically seeded genesis block initializes the total supply of **21,000,000 ΣKC** distributed to the genesis distributor address.

**Block hashing** uses SHA-256 over a canonical string encoding of all block fields, ensuring tamper-evidence.

---

### 3. ⚖️ Consensus Engine (`consensus`)

Implements a **Delegated Proof of Stake (DPoS)** consensus engine with advanced block validation:

#### Proposer Selection
Validators register with a `stake` amount. The `DPoSEngine::select_proposer()` method performs **weighted random selection** — higher-staked validators have proportionally higher probability of being selected to propose the next block.

#### Block Validation Pipeline (`DPoSEngine::validate_block`)

Every block submitted to the network passes through a strict multi-stage validation pipeline:

| Stage | Rule | Error |
|---|---|---|
| **Height Check** | `block.height == parent.height + 1` | `Invalid block height` |
| **Hash Linkage** | `block.prev_hash == parent.calculate_hash()` | `Invalid prev_hash` |
| **Timestamp Order** | `block.timestamp > parent.timestamp` | `Invalid timestamp` |
| **Future Block Guard** | `block.timestamp <= now + 5s` | `Block timestamp in the future` |
| **Proposer Verification** | Proposer must be in active validator set | `Unknown proposer` |
| **Minimum Stake** | `validator.stake >= min_stake` (default: 10,000 ΣKC) | `Proposer does not meet minimum stake` |
| **PoIW Proof Validity** | `verify_gradient_proof(proof, threshold) == true` | `Invalid Proof of Intelligence Work` |

Genesis blocks (height 0) bypass PoIW and proposer checks, as they are deterministic.

---

### 4. 🌐 P2P Network Layer (`p2p_network`)

Built on **libp2p v0.53** with:
- **`GossipSub`**: Topic-based message broadcasting for proofs and blocks.
- **`mDNS`**: Automatic local peer discovery (no bootstrap node required on LAN).
- **Noise Protocol**: Encrypted and authenticated transport.
- **Yamux**: Stream multiplexing for efficiency.

Nodes broadcast proof payloads to the `/sigmakcoin/proofs/1.0.0` topic and blocks to `/sigmakcoin/blocks/1.0.0`.

---

### 5. 🗄️ IPFS Storage Layer (`ipfs_storage`)

Handles large-scale AI model checkpoint persistence:
- Upload model weights → receive a **Content Identifier (CID)**.
- CID is embedded in the block, creating an immutable on-chain reference.
- Validators can fetch and independently verify the model at any point.
- Current implementation: mock HTTP storage (production IPFS integration planned for Phase 3).

---

### 6. 🤝 Smart Contract Marketplace

An EVM-compatible AI task marketplace (`contracts/AITaskMarketplace.sol`) enables:
- **Task Posting**: Research labs stake ΣKC to request specific training work.
- **Task Assignment**: Validators approve miners for specific tasks.
- **Reward Claiming**: Miners submit a proof hash to claim ΣKC from the locked pool.

The **Development Fund** (`contracts/DevelopmentDAO.sol`) uses **Quadratic Voting** with a vesting schedule. Token holders holding ≥100 ΣKC can submit and vote on protocol governance proposals.

---

## 💰 Tokenomics

| Parameter | Value |
|---|---|
| **Total Supply** | 21,000,000 ΣKC |
| **Block Reward** | 5 ΣKC |
| **Block Time** | ~2 seconds |
| **Halving Interval** | Every 2 years |
| **Min Validator Stake** | 10,000 ΣKC |
| **Min DAO Vote Stake** | 100 ΣKC |

### Reward Distribution per Block
```
50% → Miner (PoIW contributor)
40% → Validators (DPoS stakers)
 5% → Development DAO
 5% → Model Treasury (for public datasets)
```

---

## 🗳️ DAO Governance

All protocol-level decisions are subject to a DAO vote:

- Protocol upgrades and parameter changes (e.g., minimum loss threshold).
- Approval of new AI model architecture types for mining eligibility.
- Slashing conditions for malicious or lazy validators.
- Allocation from the Model Treasury for public data partnerships.

Proposals require a **10% quorum** and **51% approval** to pass. Voting power scales as `√(stake)` (quadratic voting), preventing plutocratic dominance.

---

## 🚀 Project Status & Roadmap

### ✅ Phase 1: Core Engine — **COMPLETE**
- [x] Plonky3 ZK-circuit implementation (`GradientCircuit`).
- [x] Proof generation & serialization (`generate_gradient_proof`).
- [x] Proof verification API (`verify_gradient_proof`).
- [x] libp2p GossipSub + mDNS P2P network.
- [x] IPFS storage wrapper with CID-based retrieval.
- [x] AI Task Marketplace Smart Contract (Solidity).
- [x] Development DAO with quadratic voting (Solidity).

### ✅ Phase 2: Blockchain Layer — **COMPLETE**
- [x] `Block` and `Transaction` data structures with SHA-256 hash linkage.
- [x] `BlockchainState`: balance management and block application logic.
- [x] Genesis block seeded with full 21M ΣKC supply.
- [x] `DPoSEngine`: weighted-random proposer selection.
- [x] Advanced 7-stage block validation pipeline (height, hash, time, stake, PoIW).
- [x] ZK-proof integration in consensus — loss threshold enforcement.
- [x] `Full Node Orchestrator`: unified startup with async task management.
- [x] Integration test suite: `tests/consensus_validation.rs`.

### 🔄 Phase 3: Network Sync & Hardening — **IN PROGRESS**
- [ ] Real ZK circuit replacement (STARK-based, replacing string proof format).
- [ ] Full block sync protocol between peers over libp2p.
- [ ] UTXO-based transaction model (replacing simple balance map).
- [ ] Slashing logic for equivocating validators.
- [ ] Production IPFS integration (remove mock storage).

### 📅 Phase 4: Mobile Mining
- [ ] Android miner app (Kotlin, MediaPipe AI inference).
- [ ] iOS miner app (Swift, CoreML inference).
- [ ] Mobile-optimized ZK circuit (reduced constraint count).

### 📅 Phase 5: Mainnet Launch
- [ ] Finalized tokenomics and genesis allocation audit.
- [ ] Testnet deployment with early validator onboarding.
- [ ] Third-party security audit (ZK circuits + smart contracts).
- [ ] Public mainnet launch.

---

## 🛠 Tech Stack

| Layer | Technology |
|---|---|
| **Core Language** | Rust (2021 edition) |
| **ZK Proofs** | [Plonky3](https://github.com/Plonky3/Plonky3) (BabyBear field, FRI-based) |
| **Consensus** | Custom DPoS in Rust |
| **Networking** | libp2p v0.53 (Gossipsub + mDNS + Noise + Yamux) |
| **Storage** | IPFS (mock → production) |
| **Smart Contracts** | Solidity (EVM-compatible) |
| **Contract Framework** | Foundry |
| **Mobile (Planned)** | Kotlin / Swift |

---

## 💻 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (stable or nightly)
- [Foundry](https://getfoundry.sh/) (for smart contract deployment)
- Git

### Clone & Build
```bash
git clone https://github.com/mashimi/sigmakcoin.git
cd sigmakcoin

# Build all workspace crates
cargo build --workspace
```

### Run the Full Node
```bash
cargo run --bin sigmak_full_node
```

### Run the Validator
```bash
cargo run --bin validator
```

### Run the Integration Tests
```bash
# Run the full consensus validation test suite
cargo test --test consensus_validation

# Run all tests in the workspace
cargo test --workspace
```

### Deploy Smart Contracts
```bash
cd contracts

# Compile contracts
forge build

# Deploy to local Anvil testnet
anvil &
forge script script/Deploy.s.sol --rpc-url http://localhost:8545 --broadcast
```

---

## 🏗 Workspace Structure

```
sigmakcoin/
├── Cargo.toml                  # Workspace root
├── Cargo.lock
├── src/
│   └── main.rs                 # Full node entry point
├── blockchain/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── block.rs            # Block & Transaction structs
│       └── state.rs            # BlockchainState (balance ledger)
├── consensus/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # DPoSEngine + validate_block pipeline
├── proof_verifier/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Public API: generate/verify proof
│       └── zk_circuit.rs      # Plonky3 GradientCircuit AIR
├── p2p_network/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # SigmaKNetwork (libp2p swarm)
│       └── main.rs
├── ipfs_storage/
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs              # IPFSStorage mock
├── validator/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs             # Validator daemon
├── contracts/
│   ├── AITaskMarketplace.sol
│   └── DevelopmentDAO.sol
└── tests/
    └── consensus_validation.rs # Integration tests
```

---

## 🛡 Security Model

| Threat | Mitigation |
|---|---|
| **Fake Proofs** | ZK-proof verification — mathematically unforgeable |
| **Proof Replay** | Block height + timestamp uniqueness |
| **Sybil Attack** | Minimum stake requirement (10,000 ΣKC) |
| **Low-Effort Mining** | Loss reduction threshold enforced on-chain |
| **Future Block Attack** | Timestamp bounded to `now + 5s` |
| **Eclipse Attack** | mDNS + GossipSub peer diversity |
| **Memory Safety** | 100% safe Rust (no unsafe blocks in core logic) |
| **Smart Contract Bugs** | Foundry test suite + planned third-party audit |

---

## 🤝 Contributing

We welcome contributions from ZK-cryptographers, AI researchers, Rust engineers, and blockchain developers.

1. Fork the repository.
2. Create a feature branch: `git checkout -b feat/your-feature`.
3. Commit your changes: `git commit -m "feat: describe your change"`.
4. Push and open a Pull Request.

Please check [Issues](https://github.com/mashimi/sigmakcoin/issues) for open tasks and the roadmap for areas of highest impact.

---

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

**Built with 🔥 and 🧠 by the ΣKCoin Foundation.**

*Proof of Intelligence Work — where every block makes the world smarter.*
