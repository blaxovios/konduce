use std::fmt;
use sqlx::{FromRow, Error};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::uuid::Uuid;
use sqlx::types::ipnetwork::IpNetwork;

use crate::infra::db::pool::Db;

#[derive(Debug, FromRow)]
/// Because we derive the macro FromRow (sqlx), the struct fields annotation must use sqlx types
pub struct Entry {
    pub id: i64,
    pub occurred_at: DateTime<Utc>,
    pub user_id: Option<i64>,
    pub username: Option<String>,
    pub ip: Option<IpNetwork>,
    pub user_agent: Option<String>,
    pub session_id: Option<Uuid>,
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let uid = self.user_id
            .map(|u| u.to_string())
            .unwrap_or_else(|| "<null>".to_string());

        let username = self.username
            .as_deref()
            .unwrap_or("<null>");

        let ip = self.ip
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "<null>".to_string());

        let ua = self.user_agent
            .as_deref()
            .unwrap_or("<null>");

        let sid = self.session_id
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| "<null>".to_string());

        write!(
            f,
            "id={id} | occurred_at={ts} | user_id={uid} | username={user} | ip={ip} | session_id={sid} | ua={ua}",
            id = self.id,
            ts = self.occurred_at,
            user = username,
            ip = ip,
            sid = sid,
            ua = ua,
        )
    }
}

pub async fn read_logs_from_db(db: &Db) -> Result<Vec<Entry>, Error> {
    let rows = sqlx::query_as::<_, Entry>(
        r#"
        SELECT id, occurred_at, user_id, username, ip, user_agent, session_id
        FROM ops.client_connect_log
        ORDER BY occurred_at DESC
        LIMIT 10
        "#,
    )
    .fetch_all(&**db) // &**db because Db derefs to PgPool
    .await?;

    Ok(rows)
}
