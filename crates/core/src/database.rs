use r2d2_sqlite::SqliteConnectionManager;
use r2d2::Pool;
use rusqlite::Result;

pub type DbPool = Pool<SqliteConnectionManager>;

// This function will create the database pool.
pub fn create_pool() -> Result<DbPool, r2d2::Error> {
    let manager = SqliteConnectionManager::file("inventory.db");
    let pool = Pool::new(manager)?;
    // Set up the table if it doesn't exist, now with a last_edited column.
    pool.get()?.execute(
        "CREATE TABLE IF NOT EXISTS ingredients (
            name        TEXT PRIMARY KEY,
            quantity    INTEGER NOT NULL,
            unit        TEXT NOT NULL,
            last_edited TEXT NOT NULL
        )",
        [],
    ).unwrap(); // Using unwrap here for simplicity, as failure on init is critical.
    Ok(pool)
}
