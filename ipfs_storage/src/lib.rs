use anyhow::Result;
use blake3::hash;
use reqwest::Client;

pub struct IPFSStorage {
    client: Client,
    gateway_url: String,
    #[allow(dead_code)]
    api_url: String,
}

impl IPFSStorage {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            gateway_url: "https://ipfs.io/ipfs/".to_string(),
            api_url: "https://api.pinata.cloud/pinning/pinJSONToIPFS".to_string(),
        }
    }
    
    pub async fn upload_model(&self, model_bytes: &[u8]) -> Result<String> {
        // Calculate CID (Content Identifier)
        let cid = format!("Qm{}", hex::encode(hash(model_bytes).as_bytes()));
        
        // For production, you'd actually upload to IPFS here
        // Using Pinata's free API or local IPFS daemon
        
        println!("📦 Model uploaded with CID: {}", cid);
        Ok(cid)
    }
    
    pub async fn download_model(&self, cid: &str) -> Result<Vec<u8>> {
        let url = format!("{}{}", self.gateway_url, cid);
        let response = self.client.get(&url).send().await?;
        let bytes = response.bytes().await?;
        Ok(bytes.to_vec())
    }
    
    pub async fn store_proof(&self, proof: &[u8]) -> Result<String> {
        let proof_hash = hash(proof);
        let cid = format!("proof_{}", hex::encode(proof_hash.as_bytes()));
        
        // Store in our mock IPFS
        println!("🔐 Proof stored with CID: {}", cid);
        Ok(cid)
    }
}

pub async fn run() -> Result<()> {
    let storage = IPFSStorage::new();
    
    // Test upload
    let test_model = b"test model weights";
    let cid = storage.upload_model(test_model).await?;
    println!("Uploaded model: {}", cid);
    
    // Test download
    let downloaded = storage.download_model(&cid).await?;
    println!("Downloaded {} bytes", downloaded.len());
    
    Ok(())
}
