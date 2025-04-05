mod config;
mod leetcode_api_runner;

use clap::{
    Parser,
    Subcommand,
};
use config::Config;
use leetcode_api_runner::LeetcodeApiRunner;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "A cli to interact with leetcode.")]
struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands
{
    Info
    {
        #[arg(short = 'i', long)]
        id: u32,
    },
    Start
    {
        #[arg(short = 'i', long)]
        id: u32,

        #[arg(short = 'l', long = "lang")]
        language: String,
    },
    Submit
    {
        #[arg(short = 'i', long)]
        id: u32,

        #[arg(short = 'p', long = "file")]
        path_to_file: String,
    },
}

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
        },
        Commands::Submit {
            id,
            path_to_file,
        } => {
            println!("Problem ID: {}", id);
            println!("Path to file: {}", path_to_file);
        },
    }
    Ok(())
}
