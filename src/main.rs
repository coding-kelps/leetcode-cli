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
    let api_runner = LeetcodeApiRunner::new(&mut config).await;

    match &cli.command {
        Commands::Info {
            id,
        } => {
            println!("{}", api_runner.get_problem_info(*id).await?);
        },
        Commands::Start {
            id,
            language,
        } => {
            let language = utils::parse_programming_language(language);
            println!(
                "{}\nHappy Coding :)",
                api_runner.start_problem(*id, language).await?
            );
        },
        Commands::Test {
            id,
            path_to_file,
        } => {
            let test_result =
                api_runner.test_response(*id, path_to_file.clone()).await?;
            println!("Test result: {}", test_result);
        },
        Commands::Submit {
            id,
            path_to_file,
        } => {
            let submit_result = api_runner
                .submit_response(*id, path_to_file.clone())
                .await?;
            println!("Submit result: {}", submit_result);
        },
        Commands::Debug {} => {
            println!("{:#?}", config.clone());
        },
    }
    Ok(())
}
