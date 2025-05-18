mod cli;
mod config;
mod leetcode_api_runner;
mod utils;

use clap::Parser;
use cli::{
    Cli,
    Commands,
};
use config::Config;
use leetcode_api_runner::LeetcodeApiRunner;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut config = Config::new();
    config.status().await?;
    let api_runner = LeetcodeApiRunner::new(config).await;

    match &cli.command {
        Commands::Info {
            id,
        } => {
            let info = api_runner.get_problem_info(*id).await;
            match info {
                Ok(info) => println!("{}", info),
                Err(e) => eprintln!("Error: {}", e),
            }
        },
        Commands::Start {
            id,
            language,
        } => {
            let language = utils::parse_programming_language(language);
            api_runner.start_problem(*id, language).await?;
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
