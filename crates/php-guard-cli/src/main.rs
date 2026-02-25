use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "php-guard")]
#[command(author, version, about = "PHP source code encryption tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Encrypt PHP files")]
    Encrypt {
        #[arg(required = true)]
        paths: Vec<String>,
        #[arg(short, long)]
        output: Option<String>,
    },
    #[command(about = "Check if files are encrypted")]
    Check {
        #[arg(required = true)]
        paths: Vec<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Encrypt { paths, output } => {
            commands::encrypt(&paths, output.as_deref())?;
        }
        Commands::Check { paths } => {
            commands::check(&paths)?;
        }
    }

    Ok(())
}
