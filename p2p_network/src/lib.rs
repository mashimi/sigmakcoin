use libp2p::{
    gossipsub, identity, mdns, noise,
    swarm::{Swarm, SwarmEvent, NetworkBehaviour},
    tcp, yamux, PeerId, Transport,
};
use serde::{Serialize, Deserialize};
use std::error::Error;
use futures::stream::StreamExt; // Needed for select_next_some

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    NewProof {
        miner_id: String,
        proof_hash: String,
        model_cid: String,
        loss_reduction: f32,
    },
    NewBlock {
        block_height: u64,
        validator_id: String,
        transactions: Vec<String>,
    },
    ValidatorAnnouncement {
        validator_id: String,
        stake: u64,
    },
}

// Combined behaviour for gossipsub and mDNS
#[derive(NetworkBehaviour)]
struct SigmaKBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

pub struct SigmaKNetwork {
    swarm: Swarm<SigmaKBehaviour>,
    topic: gossipsub::TopicHash,
}

impl SigmaKNetwork {
    pub async fn new(peer_id: PeerId, keypair: identity::Keypair) -> Result<Self, Box<dyn Error + Send + Sync>> {
        // Configure gossipsub
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(std::time::Duration::from_secs(1))
            .validation_mode(gossipsub::ValidationMode::Strict)
            .build()
            .map_err(|e| Box::new(e) as Box<dyn Error + Send + Sync>)?;
        
        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(keypair.clone()),
            gossipsub_config,
        )?;
        
        // Create our topic
        let topic = gossipsub::IdentTopic::new("sigmak_mainnet");
        gossipsub.subscribe(&topic)?;
        let topic_hash = topic.hash();
        
        // Setup mDNS
        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id)?;
        
        let behaviour = SigmaKBehaviour { gossipsub, mdns };
        
        // Setup transport
        let transport = tcp::tokio::Transport::new(tcp::Config::default())
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&keypair)?)
            .multiplex(yamux::Config::default())
            .boxed();
        
        let swarm = Swarm::new(transport, behaviour, peer_id, libp2p::swarm::Config::with_tokio_executor());
        
        Ok(Self {
            swarm,
            topic: topic_hash,
        })
    }
    
    pub async fn run(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Start listening on a random port
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        
        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => {
                    match event {
                        SwarmEvent::NewListenAddr { address, .. } => {
                            println!("📡 Listening on: {}", address);
                        }
                        SwarmEvent::Behaviour(SigmaKBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                            message, ..
                        })) => {
                            if let Ok(msg) = bincode::deserialize::<NetworkMessage>(&message.data) {
                                println!("📨 Received: {:?}", msg);
                            }
                        }
                        SwarmEvent::Behaviour(SigmaKBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                            for (peer_id, addr) in list {
                                println!("🔍 Discovered peer: {} at {}", peer_id, addr);
                                let _ = self.swarm.dial(addr);
                            }
                        }
                        _ => {}
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("Shutting down...");
                    break;
                }
            }
        }
        Ok(())
    }
    
    pub fn broadcast_proof(&mut self, proof: NetworkMessage) -> Result<(), Box<dyn Error + Send + Sync>> {
        let data = bincode::serialize(&proof)?;
        self.swarm
            .behaviour_mut()
            .gossipsub
            .publish(self.topic.clone(), data)?;
        Ok(())
    }
}

pub async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    let local_key = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(local_key.public());
    let mut network = SigmaKNetwork::new(peer_id, local_key).await?;
    network.run().await
}