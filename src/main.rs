mod cli;

use clap::Parser;
use cli::Cli;

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        cli::Commands::Add { task } => {
            println!("Adding task: {}", task);
        }
        cli::Commands::List { all } => {
            if *all {
                println!("Listing all tasks:");
            } else {
                println!("Listing pending tasks:");
            }
        }
    }
}