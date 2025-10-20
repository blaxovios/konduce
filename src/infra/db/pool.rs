use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, query};
use std::env;
use std::time::Duration;
use anyhow::{Context, Result};
use colored::Colorize;
use tracing::info;
use inquire::{Confirm, InquireError};

use crate::utils::cli_utils::prompt_db_construction;

#[derive(Clone)]
pub struct Db(pub PgPool);

impl Db {
    pub async fn connect_from_env<'a>(db_constructor: Option<String>) -> Result<Db> {
        let _ = dotenv::dotenv();

        let url = match db_constructor {
            Some(s) => s,
            None => env::var("DATABASE_URL").context("DATABASE_URL must be set. Fallback to .env")?,
        };

        let pool = PgPoolOptions::new()
            .max_connections(env_max_conns())
            .acquire_timeout(Duration::from_secs(10))
            .connect(&url)
            .await?;

        info!("Database pool ready");
        Ok(Db(pool))
    }
}

fn env_max_conns() -> u32 {
    std::env::var("DB_MAX_CONNS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(5)
}

impl std::ops::Deref for Db {
    type Target = PgPool;
    fn deref(&self) -> &Self::Target { &self.0 }
}

#[async_trait::async_trait]
pub trait DatabaseConnection {
    async fn execute(&self, query: &str) -> Result<(), sqlx::Error>;
}

#[async_trait::async_trait]
impl DatabaseConnection for Db {
    async fn execute(&self, query_str: &str) -> Result<(), sqlx::Error> {
        query(query_str)
            .execute(&self.0)
            .await?; // Await the async future here
        Ok(())
    }
}

pub async fn connect_db_with_retry() -> Result<Db> {
    loop {
        match prompt_db_construction().await {
            Ok(db_constructor) => {
                match Db::connect_from_env(Some(db_constructor.clone())).await {
                    Ok(db) => {
                        println!("{}", "Database connected".green());
                        return Ok(db);
                    }
                    Err(e) => {
                        eprintln!(
                            "{} {e}\n{}",
                            "Connection failed with provided args:".red(),
                            "Please try again or press Ctrl-C to cancel."
                        );
                        continue; // re-prompt
                    }
                }
            }
            Err(_) => {
                // Ask whether to use env-based default (Y/n)
                let use_env = Confirm::new("Wrong credentials. Use defaults from env?")
                    .with_default(true) // Enter = Yes
                    .with_help_message("Press Enter for Yes, or 'n' to re-enter credentials.")
                    .prompt();

                match use_env {
                    Ok(true) => {
                        match Db::connect_from_env(None).await {
                            Ok(db) => {
                                println!("{}", "Database connected (env)".green());
                                return Ok(db);
                            }
                            Err(e) => {
                                eprintln!("{} {e}", "Failed to connect using DATABASE_URL from env:".red());
                                // fall through to retry credential prompts
                                continue;
                            }
                        }
                    }
                    Ok(false) => {
                        println!("{}", "Okay, let's try again…".yellow());
                        continue; // re-prompt credentials
                    }
                    Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => {
                        println!("{}", "Exiting…".yellow());
                        return Err(anyhow::anyhow!("canceled by user"));
                    }
                    Err(e) => {
                        eprintln!("Prompt error: {e}");
                        continue; // re-prompt on transient prompt errors
                    }
                }
            }
        }
    }
}
