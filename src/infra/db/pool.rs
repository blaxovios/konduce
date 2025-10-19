use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, query};
use std::time::Duration;
use anyhow::Result;

#[derive(Clone)]
pub struct Db(pub PgPool);

impl Db {
    pub async fn connect_from_env() -> Result<Db> {
        let _ = dotenv::dotenv();

        let url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL is required (e.g. postgres://app_rw:pwd@localhost:5432/game_test)");

        let pool = PgPoolOptions::new()
            .max_connections(env_max_conns())
            .acquire_timeout(Duration::from_secs(10))
            .connect(&url)
            .await?;

        println!("Database pool ready");
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
