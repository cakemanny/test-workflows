use std::path::PathBuf;

use clap::{Parser, Subcommand, Args};
use dagger_sdk::HostDirectoryOptsBuilder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Prints hello and the go version
    Hello,

    /// Lists the contents of the build directory
    Ls,

    /// Runs the CI pipeline
    Ci(CiArgs)
}

#[derive(Args, Debug)]
struct CiArgs {
    /// What comes from `${{ github.event_name }}`
    #[arg(long)]
    event_name: String,

    /// What comes from `${{ github.event_path }}`. Contains the webhook payload.
    #[arg(long)]
    event_path: PathBuf,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let args = Cli::parse();

    dagger_sdk::connect(|client| async move {
        match args.command {
            Commands::Hello => {
                let version = client
                    .container()
                    .from("golang:1.19")
                    .with_exec(vec!["go", "version"])
                    .stdout()
                    .await?;

                println!("Hello from Dagger and {}", version.trim());
            }
            Commands::Ls => {
                let host_src_dir = client.host().directory_opts(
                    ".",
                    HostDirectoryOptsBuilder::default()
                        .exclude(vec!["target/", ".git/"])
                        .build()?,
                );
                let listing = client
                    .container()
                    .from("alpine:latest")
                    .with_workdir("/app")
                    .with_directory("/app", host_src_dir)
                    .with_exec(vec!["ls", "-l"])
                    .stdout()
                    .await?;
                println!("ls -l /app:\n{}", listing);
            }
            Commands::Ci(CiArgs{ event_name, event_path }) => {
                let github_token = std::env::var("GITHUB_TOKEN")?;
                println!("event_name: {event_name:?}");
                println!("event_path: {event_path:?}");
                println!("TODO: do something with octocrab");
            }
        }

        Ok(())
    })
    .await?;

    Ok(())
}
