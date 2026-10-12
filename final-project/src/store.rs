use std::path::Path;

use anyhow::{Context, Result};
use rusqlite::{params, Connection};

use crate::task::{Priority, Status, Task};

const SCHEMA: &str = "CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    status TEXT NOT NULL,
    priority TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
)";

/// SQLite-backed task storage.
///
/// Opens (and auto-creates) the database file on construction.
pub struct Store {
    conn: Connection,
}

impl Store {
    /// Open a database file, creating it and its schema if needed.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let conn = Connection::open(path)
            .with_context(|| format!("failed to open database at {}", path.display()))?;
        let store = Self { conn };
        store.init()?;
        Ok(store)
    }

    /// Open an in-memory database. Handy for tests and demos.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory().context("failed to open in-memory database")?;
        let store = Self { conn };
        store.init()?;
        Ok(store)
    }

    fn init(&self) -> Result<()> {
        self.conn
            .execute_batch(SCHEMA)
            .context("failed to initialise tasks table")?;
        Ok(())
    }

    /// Insert a new task with `todo` status and return the stored row.
    pub fn add_task(&self, title: &str, priority: Priority) -> Result<Task> {
        if title.trim().is_empty() {
            anyhow::bail!("title must not be empty");
        }
        self.conn
            .execute(
                "INSERT INTO tasks (title, status, priority) VALUES (?1, ?2, ?3)",
                params![title, Status::Todo.to_string(), priority.to_string()],
            )
            .context("failed to insert task")?;
        let id = self.conn.last_insert_rowid();
        self.get_task(id)
    }

    fn get_task(&self, id: i64) -> Result<Task> {
        self.conn
            .query_row(
                "SELECT id, title, status, priority, created_at FROM tasks WHERE id = ?1",
                params![id],
                parse_task_row,
            )
            .with_context(|| format!("failed to load task {id} after insert"))
    }

    /// List tasks, optionally filtered by status and/or priority.
    pub fn list_tasks(
        &self,
        status: Option<Status>,
        priority: Option<Priority>,
    ) -> Result<Vec<Task>> {
        let mut sql = String::from("SELECT id, title, status, priority, created_at FROM tasks");
        let mut clauses: Vec<&str> = Vec::new();
        if status.is_some() {
            clauses.push("status = ?");
        }
        if priority.is_some() {
            clauses.push("priority = ?");
        }
        if !clauses.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&clauses.join(" AND "));
        }
        sql.push_str(" ORDER BY id ASC");

        let filter_values: Vec<String> = status
            .map(|s| s.to_string())
            .into_iter()
            .chain(priority.map(|p| p.to_string()))
            .collect();

        let mut stmt = self
            .conn
            .prepare(&sql)
            .context("failed to prepare task listing query")?;
        let tasks = stmt
            .query_map(
                rusqlite::params_from_iter(filter_values.iter()),
                parse_task_row,
            )
            .context("failed to query tasks")?
            .collect::<Result<Vec<Task>, rusqlite::Error>>()
            .context("failed to read task rows")?;
        Ok(tasks)
    }

    /// Mark a task as done. Errors when the id does not exist.
    pub fn mark_done(&self, id: i64) -> Result<()> {
        let changed = self
            .conn
            .execute(
                "UPDATE tasks SET status = 'done' WHERE id = ?1",
                params![id],
            )
            .with_context(|| format!("failed to mark task {id} as done"))?;
        if changed == 0 {
            anyhow::bail!("no task found with id {id}");
        }
        Ok(())
    }

    /// Delete a task. Errors when the id does not exist.
    pub fn remove_task(&self, id: i64) -> Result<()> {
        let changed = self
            .conn
            .execute("DELETE FROM tasks WHERE id = ?1", params![id])
            .with_context(|| format!("failed to remove task {id}"))?;
        if changed == 0 {
            anyhow::bail!("no task found with id {id}");
        }
        Ok(())
    }
}

fn parse_task_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<Task> {
    let status_text: String = row.get(2)?;
    let priority_text: String = row.get(3)?;
    let status: Status = status_text.parse().map_err(|_| {
        rusqlite::Error::FromSqlConversionFailure(
            2,
            rusqlite::types::Type::Text,
            format!("invalid status value `{status_text}`").into(),
        )
    })?;
    let priority: Priority = priority_text.parse().map_err(|_| {
        rusqlite::Error::FromSqlConversionFailure(
            3,
            rusqlite::types::Type::Text,
            format!("invalid priority value `{priority_text}`").into(),
        )
    })?;
    Ok(Task {
        id: row.get(0)?,
        title: row.get(1)?,
        status,
        priority,
        created_at: row.get(4)?,
    })
}
