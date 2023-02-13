use std::path::Path;
use clap::{Args, Parser, Subcommand};

use rusvila::setup;

#[derive(Subcommand, Debug)]
enum Commands {
    Init(Init),
}

#[derive(Args, Debug)]
#[command(about = "initialise a new blog project")]
struct Init {
    #[arg(short = 'n', long = "name")]
    #[arg(required = true)]
    name: String,
    #[arg(short = 'p', long = "path")]
    #[arg(required = false)]
    path: String,
}

#[derive(Parser, Debug)]
#[command(name = "Rusvila")]
#[command(author = "Philipp Armingeon <philipp.armingeon@googlemail.com>")]
#[command(version = "1.0")]
#[command(about = "Rusvila is a tool for quickly deploying static blogs.")]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = CLI::parse();
    match &cli.command {
        Commands::Init(init) => {
            let path = Path::new(&init.path);
            setup(&init.name, &path);
        }
    }

}
