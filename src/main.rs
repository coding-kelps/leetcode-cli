use clap::Parser;
use leetcode_cli::{
    utils::{
        parse_programming_language,
        prompt_for_language,
        spin_the_spinner,
        stop_and_clear_spinner,
    },
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
    let api_runner = LeetcodeApiRunner::new(&rcs).await?;

    match &cli.command {
        Commands::Info { id } => {
            let spin = spin_the_spinner("Fetching problem info...");
            let result = api_runner.get_problem_info(*id).await;
            stop_and_clear_spinner(spin);
            match result {
                Ok(info) => println!("{}", info),
                Err(e) => eprintln!("Error fetching problem info: {}", e),
            }
        },
        Commands::Start { id, language } => {
            let default_lang = rcs
                .config
                .default_language
                .clone()
                .unwrap_or_else(|| "not found".to_string());
            let mut lang = match language {
                Some(lang) => parse_programming_language(lang),
                None => parse_programming_language(&default_lang),
            };
            let spin = spin_the_spinner("Gathering problem info...");
            let problem_name = api_runner.get_problem_name(*id).await?;
            let available_languages =
                api_runner.get_available_languages(&id).await?;
            stop_and_clear_spinner(spin);
            while lang.is_err() {
                lang = prompt_for_language(
                    id,
                    &problem_name,
                    &available_languages,
                )
                .and_then(|lang| parse_programming_language(&lang));
            }
            let lang = lang.unwrap();
            let spin = spin_the_spinner("Starting problem setup...");
            let start_problem = api_runner.start_problem(*id, lang).await;
            stop_and_clear_spinner(spin);
            match start_problem {
                Ok((success_message, _, warning)) => {
                    if let Some(warning) = warning {
                        eprintln!("{}", warning);
                    }
                    println!("{}", success_message);
                    println!("\n\nHappy coding :)");
                },
                Err(e) => eprintln!("Error starting problem: {}", e),
            }
        },
        Commands::Test { id, path_to_file } => {
            let spin = spin_the_spinner("Running tests...");
            let test_result =
                api_runner.test_response(*id, &path_to_file.clone()).await;
            stop_and_clear_spinner(spin);
            match test_result {
                Ok(_) => println!("Test result"),
                Err(e) => eprintln!("Error running tests: {}", e),
            }
        },
        Commands::Submit { id, path_to_file } => {
            let spin = spin_the_spinner("Submitting solution...");
            let submit_result =
                api_runner.submit_response(*id, &path_to_file.clone()).await;
            stop_and_clear_spinner(spin);
            match submit_result {
                Ok(_) => println!("Submit result"),
                Err(e) => eprintln!("Error submitting solution: {}", e),
            }
        },
    }
    Ok(())
}
