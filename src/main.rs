use clap::{Parser, Subcommand};

use book_api::{APP_NAME, CliError, CliResult, start_server};

#[derive(Parser)]
#[clap(
    name = APP_NAME,
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
)]
// #[clap(about = "A fictional versioning CLI", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start server
    #[clap(about = "Start HTTP server", long_about = None)]
    Serve,
}
#[tokio::main]
async fn main() -> CliResult<()> {
    let args = Cli::parse();
    match &args.commands {
        Commands::Serve => start_server()
            .await
            .map_err(|err| CliError::ServerError(err.to_string())),
    }
}
