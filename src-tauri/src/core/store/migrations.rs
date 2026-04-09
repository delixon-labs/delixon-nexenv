use rusqlite::Connection;

use crate::core::error::NexenvError;

pub const MIGRATIONS: &[(u32, &str, &str)] = &[
    (1, "initial", include_str!("../../../migrations/001_initial.sql")),
];

pub fn run_migrations(conn: &Connection) -> Result<(), NexenvError> {
    // Crear tabla de migraciones si no existe
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            version     INTEGER PRIMARY KEY,
            name        TEXT NOT NULL,
            applied_at  TEXT NOT NULL DEFAULT (datetime('now'))
        );"
    ).map_err(|e| NexenvError::Database(e.to_string()))?;

    for (version, name, sql) in MIGRATIONS {
        let applied: bool = conn
            .prepare("SELECT COUNT(*) FROM _migrations WHERE version = ?1")
            .and_then(|mut stmt| stmt.query_row([version], |row| row.get::<_, i64>(0)))
            .map(|count| count > 0)
            .unwrap_or(false);

        if !applied {
            conn.execute_batch(sql)
                .map_err(|e| NexenvError::Database(format!("Migration {} ({}): {}", version, name, e)))?;

            conn.execute(
                "INSERT INTO _migrations (version, name) VALUES (?1, ?2)",
                rusqlite::params![version, name],
            ).map_err(|e| NexenvError::Database(e.to_string()))?;
        }
    }

    Ok(())
}
