use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};
use dagger_sdk::HostDirectoryOptsBuilder;
use octocrab::models::webhook_events::{WebhookEvent, WebhookEventPayload};
use tracing::info;

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
    Ci(CiArgs),
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

    // Ensure args are valid before spinning up the dagger engine.
    let args = Cli::parse();

    dagger_sdk::connect(|client| async move { dispatch(args, client).await }).await?;

    Ok(())
}

async fn dispatch(args: Cli, client: dagger_sdk::DaggerConn) -> eyre::Result<()> {
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
        Commands::Ci(ci_args) => {
            ci_pipeline(ci_args, client).await?;
        }
    }
    Ok(())
}

async fn ci_pipeline(
    CiArgs {
        event_name,
        event_path,
    }: CiArgs,
    _client: dagger_sdk::DaggerConn,
) -> eyre::Result<()> {
    let github_token = std::env::var("GITHUB_TOKEN")?;

    let event_body = std::fs::read_to_string(event_path)?;

    let event = WebhookEvent::try_from_header_and_body(&event_name, &event_body)?;

    match event.specific {
        WebhookEventPayload::PullRequest(payload) => {
            info!(event = &event_name, "handling event");

            let Some(repo) = event.repository else {
                eyre::bail!("pull_request event without repository");
            };

            info!("plus one-ing the PR!");
            octocrab::instance()
                .user_access_token(github_token)?
                .issues_by_id(repo.id)
                .create_reaction(payload.number,
                    octocrab::models::reactions::ReactionContent::PlusOne)
                .await?;

        },
        _ => info!(event = %event_name, "ignoring event"),
    }

    Ok(())
}
