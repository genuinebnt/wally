use std::fmt;

use clap::{arg, Parser, Subcommand};

use crate::api::fetch_wallpapers;

#[derive(Parser)]
#[clap(name = "wallhaven-cli", version = "0.1.0", author = "Genuine")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Index {
        #[command(flatten)]
        params: Params,
    },
}

#[derive(Parser, Debug, Clone)]
pub struct Params {
    #[arg(long, env = "WH_APIKEY", default_value = Some(""))]
    pub api_key: Option<String>,

    #[arg(long, env = "WH_QUERY", default_value = Some(""))]
    pub query: Option<String>,

    #[arg(long, env = "WH_PAGE", default_value = Some("1"))]
    pub page: Option<usize>,

    #[arg(long, env = "WH_CATEGORY", default_value = Some("010"))]
    pub category: Option<String>,

    #[arg(long, env = "WH_SORT", default_value = Some("views"))]
    pub sort: Option<String>,

    #[arg(long, env = "WH_ORDER", default_value = Some("desc"))]
    pub order: Option<String>,

    #[arg(long, env = "WH_ATLEAST", default_value = Some("1920x1080"))]
    pub atleast: Option<String>,

    #[arg(long, env = "WH_RATIO", default_value = Some("16x9,16x10"))]
    pub ratio: Option<String>,

    #[arg(long, env = "WH_PURITY", default_value = Some("100"))]
    pub purity: Option<usize>,
}

// Define the CliError type
#[derive(Debug)]
pub enum CliError {
    EnvError(dotenv::Error),
    RequestError(reqwest::Error),
}

impl From<dotenv::Error> for CliError {
    fn from(err: dotenv::Error) -> CliError {
        CliError::EnvError(err)
    }
}

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> CliError {
        CliError::RequestError(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for CliError {}

impl Cli {
    pub async fn run_cli(&self) -> Result<(), CliError> {
        dotenv::dotenv()?;
        match &self.command {
            Commands::Index { params } => {
                let wallpapers = fetch_wallpapers(params.clone()).await?;
                println!("{:#?}", wallpapers.data);
            }
        }

        Ok(())
    }
}
