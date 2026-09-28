use clap::Parser;
use leetcode_cli::{
    login::run_login,
    utils::{
        parse_programming_language,
        prompt_for_language,
        spin_the_spinner,
        stop_and_clear_spinner,
    },
    Cli,
    Commands,
    LeetcodeApiRunner,
    LocalConfig,
    RuntimeConfigSetup,
};

/// Creates the api client, and if the saved token is expired or invalid,
/// transparently re-runs the browser login to get a fresh one before
/// retrying. The persisted browser profile usually makes the re-login
/// instant, without any manual action.
async fn build_api_runner(
    rcs: &mut RuntimeConfigSetup,
) -> Result<Option<LeetcodeApiRunner>, Box<dyn std::error::Error>> {
    if let Ok(api_runner) = LeetcodeApiRunner::new(rcs).await {
        return Ok(Some(api_runner));
    }

    if rcs.config.leetcode_token.is_empty() {
        eprintln!(
            "No LeetCode token found in {}.\nOpening the browser to log in \
             once and save it automatically...",
            rcs.config_file.display()
        );
    } else {
        eprintln!(
            "Your saved LeetCode token looks expired or invalid.\nOpening the \
             browser to refresh it (log in if prompted)..."
        );
    }
    let spin = spin_the_spinner("Waiting for leetcode login...");
    let result = run_login(rcs).await;
    stop_and_clear_spinner(spin);
    match result {
        Ok(message) => println!("{message}"),
        Err(e) => {
            eprintln!("Automatic token refresh failed: {e}");
            eprintln!("Run `leetcode-cli login` to log in manually.");
            return Ok(None);
        },
    }

    rcs.status()?;
    match LeetcodeApiRunner::new(rcs).await {
        Ok(api_runner) => Ok(Some(api_runner)),
        Err(e) => {
            eprintln!("Error creating the API client after re-login: {e}");
            Ok(None)
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut rcs = RuntimeConfigSetup::new();
    rcs.status()?;

    if let Commands::Login { browser } = &cli.command {
        if let Some(browser) = browser {
            std::env::set_var("LEETCODE_CLI_BROWSER", browser);
        }
        let spin = spin_the_spinner("Waiting for leetcode login...");
        let result = run_login(&rcs).await;
        stop_and_clear_spinner(spin);
        match result {
            Ok(message) => println!("{message}"),
            Err(e) => eprintln!("Login failed: {e}"),
        }
        return Ok(());
    }

    let Some(api_runner) = build_api_runner(&mut rcs).await? else {
        return Ok(());
    };

    match &cli.command {
        Commands::Info { id } => {
            let spin = spin_the_spinner("Fetching problem info...");
            let result = api_runner.get_problem_info(*id).await;
            stop_and_clear_spinner(spin);
            match result {
                Ok(info) => println!("{info}"),
                Err(e) => eprintln!("Error fetching problem info: {e}"),
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
                api_runner.get_available_languages(id).await?;
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
                Ok((success_message, pb_dir, warning)) => {
                    if let Some(warning) = warning {
                        eprintln!("{warning}");
                    }
                    println!("{success_message}");
                    println!("\nHappy coding :)");
                    println!(
                        "\n(ps: to use local config feature, you should \ncd \
                         {}\n;)",
                        pb_dir.display()
                    );
                },
                Err(e) => eprintln!("Error starting problem: {e}"),
            }
        },
        Commands::Test { id, path_to_file } => {
            let (problem_id, file_path) =
                LocalConfig::resolve_problem_params(*id, path_to_file.clone())?;

            let spin = spin_the_spinner("Running tests...");
            let test_result =
                api_runner.test_response(problem_id, &file_path).await;
            stop_and_clear_spinner(spin);
            match test_result {
                Ok(message) => println!("{message}"),
                Err(e) => eprintln!("Error running tests:\n{e}"),
            }
        },
        Commands::Submit { id, path_to_file } => {
            let (problem_id, file_path) =
                LocalConfig::resolve_problem_params(*id, path_to_file.clone())?;

            let spin = spin_the_spinner("Submitting solution...");
            let submit_result =
                api_runner.submit_response(problem_id, &file_path).await;
            stop_and_clear_spinner(spin);
            match submit_result {
                Ok(_) => println!("Submit result"),
                Err(e) => eprintln!("Error submitting solution: {e}"),
            }
        },
        Commands::Login { .. } => {
            unreachable!("login is handled before the command match")
        },
    }
    Ok(())
}
