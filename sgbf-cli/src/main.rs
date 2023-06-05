use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli: Root = Root::parse();
    if let Err(err) = handle_command(cli).await {
        eprintln!("{err}");
        return Err(err)
    }
    Ok(())
}

async fn handle_command(cli: Root) -> Result<()> {
    
}
