use clap::{
    Parser,
    Subcommand,
};
#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "A cli to interact with leetcode.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Info {
        #[arg(short = 'i', long)]
        id: u32,
    },
    Start {
        #[arg(short = 'i', long)]
        id: u32,

        #[arg(short = 'l', long = "lang")]
        language: Option<String>,
    },
    Test {
        #[arg(short = 'i', long)]
        id:           Option<u32>,
        #[arg(short = 'p', long = "file")]
        path_to_file: Option<String>,
    },
    Submit {
        #[arg(short = 'i', long)]
        id: Option<u32>,

        #[arg(short = 'p', long = "file")]
        path_to_file: Option<String>,
    },
}
