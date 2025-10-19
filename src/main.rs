use std::process::exit;
use std::error::Error as StdError;
use clap::Parser;
use human_panic::setup_panic;
use colored::Colorize;

mod infra;
mod utils;
mod handlers;

use crate::infra::db::pool::Db;
use crate::handlers::{db_handler, suggester};


#[derive(Parser, Debug)]
#[command(name = env!("CARGO_PKG_NAME"), author, version, about, long_about)]
/// The Arguments struct is used to parse the command line arguments
struct Arguments {
    #[arg(required=false)]
    cmd: Option<String>,

    #[arg(short, long)]
    custom: Option<String>,

    #[arg(short, long)]
    docs: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    // Nice panic reports
    setup_panic!();

    // Load local env (no-op in prod if .env missing)
    let _ = dotenv::dotenv();

    // Connect DB (fail fast if missing/invalid)
    let db = Db::connect_from_env()
        .await
        .expect("DATABASE_URL must be set and the database reachable");

    // Parse CLI args
    let args = Arguments::parse();

    // Resolve command: CLI arg or interactive prompt with autocomplete
    let cmd = match args.cmd {
        Some(c) => c,
        None => {
            inquire::Text::new("Enter Command: ")
                .with_help_message("Enter a valid command")
                .with_autocomplete(&suggester::suggester) // <-- point to the function
                .prompt()?
        }
    };

    // Dispatch
    match cmd.as_str() {
        "get" => {
            // If you want to print results here, handle the returned data
            let _ = db_handler::get(&db).await?;
        }
        "exit" => {
            println!("{}",
            "Exiting...".yellow()
        );
            exit(0);
        }
        _ => {
            // Unknown command
            eprintln!("Unknown command: {cmd}");
            println!();
        }
    }

    Ok(())

}
