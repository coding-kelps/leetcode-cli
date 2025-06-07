use clap::Parser;
use leetcode_cli::{
    utils,
    Cli,
    Commands,
    LeetcodeApiRunner,
    RuntimeConfigSetup,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut rcs = RuntimeConfigSetup::new();
    rcs.status()?;
    let api_runner = LeetcodeApiRunner::new(rcs).await;

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
            let lang = match language {
                Some(lang) => utils::parse_programming_language(lang)?,
                None => rcs.config.default_language.clone(),
            };
            let start_problem = api_runner.start_problem(*id, lang).await?;
            println!("{}\n\nHappy coding :)", start_problem);
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
    }
    Ok(())
}
