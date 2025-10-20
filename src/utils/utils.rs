use anyhow::Result;
use urlencoding::encode;

pub fn provide_db_credentials_url(
    username: &str,
    password: &str,
    host: &str,
    port: u16,
    db_name: &str,
) -> Result<String> {
    let u = encode(username.trim());
    let p = encode(password); // important: escape specials
    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        u, p, host.trim(), port, db_name.trim()
    );
    Ok(url)
}
