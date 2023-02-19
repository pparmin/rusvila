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
    //#[arg(default_value_os_t())]
    path: Option<String>,
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
            match &init.path {
                Some(p) => {
                    let path = Path::new(&p);
                    setup(&init.name, &path);
                },
                None => {
                    let path = std::env::current_dir()
                        .expect("Error while fetching current working directory");
                    setup(&init.name, &path)
                }
            }

        }
    }

}
