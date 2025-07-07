use clap::Parser;
use leetcode_cli::{
    utils,
    Cli,
    Commands,
    LeetcodeApiRunner,
    RuntimeConfigSetup,
};
use spinners::{
    Spinner,
    Spinners,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut rcs = RuntimeConfigSetup::new();
    rcs.status()?;
    let api_runner = LeetcodeApiRunner::new(&rcs).await;

    match &cli.command {
        Commands::Info {
            id,
        } => {
            let mut spinner = Spinner::new(
                Spinners::Dots12,
                "Fetching problem info...".into(),
            );
            let result = api_runner.get_problem_info(*id).await?;
            spinner.stop();
            println!("{}", result);
        },
        Commands::Start {
            id,
            language,
        } => {
            let default = &rcs.config.default_language.unwrap();
            let lang = match language {
                Some(lang) => utils::parse_programming_language(lang)?,
                None => utils::parse_programming_language(default)?,
            };
            let mut spinner = Spinner::new(
                Spinners::Dots12,
                "Starting problem setup...".into(),
            );
            let start_problem = api_runner.start_problem(*id, lang).await?;
            spinner.stop();
            println!("{}\n\nHappy coding :)", start_problem);
        },
        Commands::Test {
            id,
            path_to_file,
        } => {
            let mut spinner =
                Spinner::new(Spinners::Dots12, "Running tests...".into());
            let test_result =
                api_runner.test_response(*id, path_to_file.clone()).await?;
            spinner.stop();
            println!("Test result: {}", test_result);
        },
        Commands::Submit {
            id,
            path_to_file,
        } => {
            let mut spinner =
                Spinner::new(Spinners::Dots12, "Submitting solution...".into());
            let submit_result = api_runner
                .submit_response(*id, path_to_file.clone())
                .await?;
            spinner.stop();
            println!("Submit result: {}", submit_result);
        },
    }
    Ok(())
}
