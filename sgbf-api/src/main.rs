use anyhow::Context;
use sgbf_api::server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    // blocking until shutdown
    server::init_default_server().await?;

    Ok(())
}