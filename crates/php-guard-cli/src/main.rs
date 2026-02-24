use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;
mod config;

#[derive(Parser)]
#[command(name = "php-guard")]
#[command(author, version, about = "PHP source code encryption tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Generate random encryption key")]
    GenerateKey {
        #[arg(short = 'H', long, default_value = "12")]
        header_length: usize,
        #[arg(short = 'K', long, default_value = "16")]
        key_length: usize,
        #[arg(short, long)]
        output: Option<String>,
    },
    #[command(about = "Encrypt PHP files")]
    Encrypt {
        #[arg(required = true)]
        paths: Vec<String>,
        #[arg(short, long)]
        output: Option<String>,
        #[arg(short = 'c', long, default_value = "php-guard.toml")]
        config: String,
    },
    #[command(about = "Check if files are encrypted")]
    Check {
        #[arg(required = true)]
        paths: Vec<String>,
        #[arg(short = 'c', long, default_value = "php-guard.toml")]
        config: String,
    },
    #[command(about = "Verify configuration consistency")]
    Verify {
        #[arg(short = 'r', long, default_value = "src/config.rs")]
        rust_config: String,
        #[arg(short = 'p', long, default_value = "tools/php-guard.php")]
        php_config: String,
    },
    #[command(about = "Initialize configuration file")]
    Init {
        #[arg(short, long, default_value = "php-guard.toml")]
        output: String,
    },
    #[command(about = "Build PHP extension")]
    Build {
        #[arg(short, long)]
        release: bool,
        #[arg(long)]
        php_config: Option<String>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateKey {
            header_length,
            key_length,
            output,
        } => {
            commands::generate_key(header_length, key_length, output)?;
        }
        Commands::Encrypt {
            paths,
            output,
            config,
        } => {
            commands::encrypt(&paths, output.as_deref(), &config)?;
        }
        Commands::Check { paths, config } => {
            commands::check(&paths, &config)?;
        }
        Commands::Verify {
            rust_config,
            php_config,
        } => {
            commands::verify(&rust_config, &php_config)?;
        }
        Commands::Init { output } => {
            commands::init(&output)?;
        }
        Commands::Build {
            release,
            php_config,
        } => {
            commands::build(release, php_config.as_deref())?;
        }
    }

    Ok(())
}
