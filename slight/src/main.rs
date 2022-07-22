use crate::commands::{run::handle_run, secret::handle_secret};
use anyhow::Result;
use clap::{Parser, Subcommand};
use commands::add::handle_add;
use spiderlightning::core::slightfile::SlightfileInfo;

mod commands;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run slight providing a config and a module
    Run {
        #[clap(short, long, value_parser)]
        module: String,
        #[clap(short, long, value_parser)]
        config: String,
    },
    /// Add a secret to the application
    Secret {
        #[clap(short, long, value_parser)]
        config: String,
        #[clap(short, long, value_parser)]
        key: String,
        #[clap(short, long, value_parser)]
        value: String,
    },
    /// Add a WIT interface to your project
    Add {
        /// Usage: `slight add kv@v0.1` for the v0.1 of the kv interface, or `slight add kv` for the latest version of the kv interface
        #[clap(short, long, value_parser)]
        interface_and_version: String,
        #[clap(short, long, value_parser)]
        auth_token: Option<String>,
    },
}

/// The entry point for wasi-cloud CLI
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let args = Args::parse();
    match &args.command {
        Commands::Run { module, config } => {
            let SlightfileInfo {
                path: toml_file_path,
                slightfile: toml,
                ..
            } = SlightfileInfo::new(config)?;
            handle_run(module, &toml, &toml_file_path)
        }
        Commands::Secret { key, value, config } => {
            let SlightfileInfo {
                file: mut toml_file,
                slightfile: mut toml,
                ..
            } = SlightfileInfo::new(config)?;
            handle_secret(key, value, &mut toml, &mut toml_file)
        }
        Commands::Add {
            interface_and_version,
            auth_token,
        } => handle_add(interface_and_version, auth_token),
    }
}
