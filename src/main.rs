mod cli;
mod config;
mod leetcode_api_runner;

use clap::Parser;
use cli::{
    Cli,
    Commands,
};
use config::Config;
use leetcode_api_runner::LeetcodeApiRunner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let cli = Cli::parse();
    let mut config = Config::new();
    config.status().await?;
    let api_runner = LeetcodeApiRunner::new(config).await;

    match &cli.command {
        Commands::Info {
            id,
        } => {
            api_runner.get_problem_info(*id).await;
        },
        Commands::Start {
            id,
            language,
        } => {
            println!("Problem ID: {}", id);
            println!("Language: {}", language);
            unimplemented!();
        },
        Commands::Submit {
            id,
            path_to_file,
        } => {
            println!("Problem ID: {}", id);
            println!("Path to file: {}", path_to_file);
            unimplemented!();
        },
    }
    Ok(())
}
