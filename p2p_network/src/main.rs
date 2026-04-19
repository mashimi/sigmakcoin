use anyhow::Result;
use p2p_network::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
