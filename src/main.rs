use clap::{
    Parser,
    Subcommand,
};

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

fn main()
{
    let cli = Cli::parse();

    // leetcode token check

    match &cli.command {
        Commands::Info {
            id,
        } => {
            println!("Problem ID: {}", id);
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
}
