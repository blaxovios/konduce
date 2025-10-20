use sqlx::Error;
use colored::Colorize;

use crate::infra::db::pool::Db;
use crate::infra::db::db_comms::{read_logs_from_db, Entry};


/// Get from db
pub async fn get(db: &Db) -> Result<Vec<Entry>, Error> {
    println!("{}",
        "Executing get command...".yellow()
    );
    let logs = read_logs_from_db(db).await?;
    println!("{}",
        "Latest 10 values from database:".green()
    );
    for (i, log) in logs.iter().enumerate() {
        println!("{:>3}. {}", i + 1, log); // uses Display impl
    }
    Ok(logs)
}