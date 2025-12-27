use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Todoooo", version = "1.0", author = "Mark Goodwin", about = "A to-do list CLI application.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        #[arg(short, long)]
        tag: Option<String>,
        #[arg(short, long)]
        priority: Option<String>,
        #[arg(short, long)]
        due: Option<String>,
    },
    List {
        #[arg(short, long)]
        all: bool,
    }
}