use clap::Parser;
use human_panic::setup_panic;
use colored::Colorize;
use anyhow::Result;
use inquire::{InquireError, Text};

mod infra;
mod utils;
mod handlers;
mod core;
mod constants;

use crate::infra::db::pool::connect_db_with_retry;
use crate::core::suggester::dispatch_or_suggest;


#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), author, version, about, long_about)]
/// The Arguments struct is used to parse the command line arguments
struct Arguments {
    #[arg(required=false)]
    cmd: Option<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    setup_panic!();
    let _ = dotenv::dotenv();

    let db = connect_db_with_retry().await?;
    let args = Arguments::parse();

    // One-shot mode
    if let Some(cmd) = args.cmd {
        return dispatch_or_suggest(&db, cmd).await;
    }

    // REPL mode
    loop {
        let input = Text::new("Enter command")
            .with_help_message("try: get | exit")
            .prompt();

        let cmd = match input {
            Ok(s) => s.trim().to_string(),
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                println!("{}", "Exiting…".yellow());
                break;
            }
            Err(e) => {
                eprintln!("Prompt error: {e}");
                continue;
            }
        };

        if cmd.is_empty() {
            continue;
        }
        if cmd.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting…".yellow());
            break;
        }

        if let Err(e) = dispatch_or_suggest(&db, cmd).await {
            eprintln!("Error: {e}");
        }
    }

    Ok(())
}
