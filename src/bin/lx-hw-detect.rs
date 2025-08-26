use clap::Parser;
use lx_hw_detect::cli::{Cli, CliHandler};
use lx_hw_detect::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let handler = CliHandler::new();
    handler.run(cli).await
}
