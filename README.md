# ΣKCoin (SigmaKCoin) 🚀

**ΣKCoin** (SigmaKCoin) is a next-generation blockchain architecture that redefines cryptocurrency mining by replacing energy-intensive computation with **Proof of Intelligence Work (PoIW)**. Instead of calculating arbitrary hashes, miners on the ΣKCoin network contribute computational power to train and optimize decentralized AI models.

---

## 🏛 Vision: From PoW to PoIW
Traditional blockchains consume vast amounts of electricity for Proof of Work (PoW). ΣKCoin redirects this energy toward solving real-world intelligence tasks. By combining **Zero-Knowledge Proofs (ZKP)** with decentralized AI training, we ensure that every watt expended adds value to the global AI ecosystem.

### Core Principles
- **Intelligence-Backed Value**: New coins are minted only when valid AI model improvements are proven.
- **Privacy-Preserving Training**: Using Homomorphic Encryption and ZK-Proofs to protect data and model integrity.
- **Mobile-First Accessibility**: Efficient circuits designed to run on high-end mobile devices and consumer GPUs.

---

## 🧩 Core Architecture

The ΣKCoin node is composed of three primary layers:

### 1. ZK-Proof Verifier (Plonky3)
The heart of our consensus mechanism. It verifies that a miner has performed valid training steps (gradient descent) on a specific model without requiring the validator to re-run the training.
- **Technology**: Built with [Plonky3](https://github.com/Plonky3/Plonky3) for high-performance recursion and hardware acceleration.
- **Constraints**: Verifies loss reduction, gradient non-zero checks, and step counter integrity.

### 2. P2P Gossip Network (libp2p)
A decentralized communication layer that synchronizes proofs, blocks, and validator announcements across the global network.
- **Protocols**: `gossipsub` for message broadcasting and `mDNS` for local peer discovery.
- **Security**: Noise-encrypted channels with Yamux multiplexing.

### 3. IPFS Storage Layer
Handles the persistence of large-scale AI model checkpoints and training datasets.
- **Integration**: Seamless CID-based retrieval of model weights.
- **Verification**: Content-addressed storage ensures that miners are training on the exact model version specified by the network.

### 4. Smart Contract Marketplace
An EVM-compatible marketplace where AI tasks are posted and rewarded.
- **Governance**: Fully decentralized via the **ΣKCoin Development DAO**.
- **Marketplace**: Users can stake ΣKC to request specific training tasks (e.g., fine-tuning a Large Language Model).

---

## 💰 Tokenomics & Governance

### Supply & Allocation
- **Total Supply**: 21,000,000 ΣKC (Bitcoin-equivalent scarcity).
- **Initial Reward**: 5 ΣKC per block.
- **Block Time**: 2 seconds.
- **Halving Interval**: Every 2 years (accelerated adoption phase).

### Reward Distribution
- **Miners (PoIW)**: 50%
- **Validators (Staked)**: 40%
- **Development DAO**: 5%
- **Model Treasury**: 5%

### DAO Governance
The **Development Fund** is controlled strictly by a DAO. Token holders with a minimum stake of 100 ΣKC can propose and vote on:
- Protocol upgrades and bug bounties.
- Approval of new AI model architectures for mining.
- Slashing conditions for malicious validators.

---

## 🚀 Roadmap

### Phase 1: Core Engine (Current)
- [x] Plonky3 ZK-Verifier basic circuits.
- [x] libp2p GossipSub network implementation.
- [x] IPFS storage wrapper.
- [x] AI Task Marketplace Smart Contract.

### Phase 2: Blockchain Layer (Upcoming)
- [ ] Block & UTXO state management.
- [ ] DPoS consensus selection algorithm.
- [ ] Genesis block distribution.

### Phase 3: Mobile & Professional UI
- [ ] Android & iOS Miner App prototype.
- [ ] Desktop Node Dashboard (NISAR Watch integration).

---

## 🛠 Tech Stack
- **Languages**: Rust (Core), Solidity (Contracts), Kotlin/Swift (Mobile).
- **ZKP**: Plonky3.
- **Networking**: libp2p.
- **Storage**: IPFS.
- **Blockchain Framework**: Custom Rust implementation with EVM bridging.

---

## 💻 Getting Started

### Prerequisites
- [Rust](https://rustup.rs/) (Nightly toolchain recommended)
- [Foundry](https://getfoundry.sh/) (For smart contract deployment)

### Running a Node
```bash
# Clone the repository
git clone https://github.com/mashimi/sigmakcoin.git
cd sigmakcoin

# Run the full node orchestrator
cargo run --bin sigmak_full_node
```

### Running the Services Separately
```bash
# Start P2P Network
cd p2p_network && cargo run

# Start IPFS Storage Wrapper
cd ipfs_storage && cargo run
```

---

## 🛡 Security & Audit
ΣKCoin utilizes hardware attestation (Android SafetyNet / iOS DeviceCheck) and ZK-SNARKs to prevent Sybil attacks and fake proof submission. All core logic is written in memory-safe Rust.

---

## 🤝 Contributing
We welcome contributions from AI researchers, ZK-cryptographers, and blockchain engineers. Please check our [Issues](https://github.com/mashimi/sigmakcoin/issues) for open tasks.

**Built with 🔥 by the ΣKCoin Foundation.**
