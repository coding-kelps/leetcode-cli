use clap::{
    Parser,
    Subcommand,
};

mod config;
use config::Config;

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
    #[command(short_flag = 'i')]
    Info
    {
        /// The problem number
        #[arg(short = 'i', long)]
        id: u32,
    },
    #[command(short_flag = 'g')]
    Get
    {
        /// The problem number
        #[arg(short = 'i', long)]
        id: u32,

        #[arg(short = 'l', long = "lang")]
        language: String,
    },
    #[command(short_flag = 's')]
    Submit
    {
        /// The problem number
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

    match &cli.command {
        Commands::Info {
            id,
        } => {
            println!("asking for info on problem ID: {}", id);
        },
        Commands::Get {
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
