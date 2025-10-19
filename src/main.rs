use clap::Parser;
use human_panic::setup_panic;
use colored::Colorize;
use anyhow::Result;

mod infra;
mod utils;
mod handlers;
mod core;
mod constants;

use crate::infra::db::pool::Db;
use crate::core::suggester;
use crate::utils::cli_utils::dispatch;


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

    let db = Db::connect_from_env().await?;

    let args = Arguments::parse();

    // If a command is provided, run it once; otherwise open an interactive loop
    if let Some(cmd) = args.cmd {
        dispatch(&db, cmd).await?;
        return Ok(());
    }

    // Simple REPL: keep asking until user types "exit"
    loop {
        let input = inquire::Text::new("Enter command")
            .with_help_message("try: get | exit")
            .with_autocomplete(&suggester::suggester)
            .prompt();

        let cmd = match input {
            Ok(s) => s.trim().to_string(),
            Err(_) => {
                println!("{}", "Exiting…".yellow());
                break;
            }
        };

        if cmd.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting…".yellow());
            break;
        }

        if let Err(e) = dispatch(&db, cmd).await {
            eprintln!("Error: {e}");
        }
    }

    Ok(())
}
