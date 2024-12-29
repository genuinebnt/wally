use clap::Parser;
use wally::cli::{Cli, CliError};

#[tokio::main]
pub async fn main() -> Result<(), CliError> {
    let cli = Cli::parse(); 
    cli.run_cli().await?;
    Ok(()) 
}