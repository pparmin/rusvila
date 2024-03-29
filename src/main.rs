use std::path::Path;

use clap::{Args, Parser, Subcommand};

use rusvila::Blog;

#[derive(Subcommand, Debug)]

enum Commands {
    Init(Init),

    #[command(about = "Build public files for deployment")]
    Build,

    #[command(about = "Remove blog project (currently for testing purposes)")]
    Remove,
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

    // project variable is for general tracking of the blog project. Currently, this only allows for one project at once
    let mut project = Blog::new();

    match &cli.command {
        Commands::Init(init) => match &init.path {
            Some(p) => {
                project.setup(&init.name, Path::new(&p));
            }

            None => {
                let path = std::env::current_dir()
                    .expect("Error while fetching current working directory");
                project.setup(&init.name, &path);
            }
        },
        Commands::Build => project.build(),

        Commands::Remove => project.remove(),
    }
}
