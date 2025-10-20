use colored::Colorize;
use inquire::{InquireError, Text};
use std::error::Error as StdError;
use std::io;

use crate::utils::utils::{provide_db_credentials_url};


pub async fn prompt_db_construction() -> Result<String, Box<dyn StdError>> {
    // Helper to prompt once with consistent handling
    let mut ask = |label: &str, help: &str| -> Result<String, Box<dyn StdError>> {
        match Text::new(label).with_help_message(help).prompt() {
            Ok(s) => Ok(s.trim().to_string()),
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                println!("{}", "Exiting…".yellow());
                Err(Box::new(io::Error::new(io::ErrorKind::Interrupted, "prompt canceled")))
            }
            Err(e) => Err(Box::new(e)),
        }
    };

    let username = ask("Username:", "Provide username:")?;
    let password = ask("Password:", "Provide password:")?;
    let host     = ask("Host:", "Provide host:")?;
    let port_str = ask("Port:", "Provide port (0–65535):")?;
    let db_name  = ask("Database name:", "Provide database name:")?;

    // Parse port safely
    let port: u16 = port_str.parse().map_err(|_| {
        Box::new(io::Error::new(io::ErrorKind::InvalidInput, "invalid port")) as Box<dyn StdError>
    })?;

    // Build connection string (Password::Text expects &str)
    let db_constructor = provide_db_credentials_url(
        &username,
        &password,
        &host,
        port,
        &db_name,
    )?;

    Ok(db_constructor)
}