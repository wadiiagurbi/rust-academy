//! SQLite setup: connection pool + embedded migrations.
//!
//! This backend is SQLite-only. Moving to Postgres/MySQL would require
//! swapping the `sqlx` driver feature, adapting the `?` placeholders and
//! `RETURNING`/`ON CONFLICT` statements, reworking the migrations, and
//! replacing `SqliteConnectOptions` handling below — it is not a
//! `DATABASE_URL`-only change.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

pub async fn connect(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str(database_url)?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
