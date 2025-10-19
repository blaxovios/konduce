use anyhow::Result;
use colored::Colorize;

use crate::infra::db::pool::Db;
use crate::handlers::db_handler;


pub async fn dispatch(db: &Db, cmd: String) -> Result<()> {
    match cmd.as_str() {
        "get" => {
            let rows = db_handler::get(db).await?;
            println!("{} {}", "Rows:".green(), rows.len());
        }
        other => {
            eprintln!("Unknown command: {other}");
            println!("Available: get | exit");
        }
    }
    Ok(())
}
