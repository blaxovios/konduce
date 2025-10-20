/// Simple rust filter map function to suggest commands based on user input.
use colored::Colorize;
use anyhow::Result;

use crate::core::did_you_mean::did_you_mean;
use crate::constants::CMD_COMMANDS;
use crate::handlers::db_handler;
use crate::infra::db::pool::Db;


pub async fn dispatch_or_suggest(db: &Db, cmd: String) -> Result<()> {
    match cmd.as_str() {
        "get" => {
            let rows = db_handler::get(db).await?;
            println!("{} {}", "Rows:".green(), rows.len());
            Ok(())
        }
        "exit" => {
            println!("{}", "Exiting…".yellow());
            Ok(())
        }
        _ => {
            // Unknown: suggest the closest known command
            if let Some((best, _d)) = did_you_mean(&cmd, CMD_COMMANDS.iter().copied()) {
                println!("{} {}", "Did you mean:".yellow(), best.green());
            } else {
                println!("Unknown command: {cmd}");
            }
            Ok(())
        }
    }
}
