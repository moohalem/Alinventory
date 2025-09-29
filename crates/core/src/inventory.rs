use crate::database::DbPool;
use rusqlite::Result;
use chrono::{DateTime, Utc};

// Add the `last_edited` field to the Ingredient struct
#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Ingredient {
    pub name: String,
    pub quantity: u32,
    pub unit: String,
    pub last_edited: DateTime<Utc>,
}

// The Inventory struct now holds the database connection pool
pub struct Inventory {
    pool: DbPool,
}

// Define custom error types for better error handling
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Database pool error: {0}")]
    Connection(#[from] r2d2::Error),
    #[error("Database query error: {0}")]
    Query(#[from] rusqlite::Error),
    #[error("Ingredient '{0}' not found")]
    NotFound(String),
}

impl Inventory {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    // This method now automatically sets the timestamp on add/update
    pub fn add_ingredient(&self, name: String, quantity: u32, unit: String) -> Result<(), Error> {
        let conn = self.pool.get()?;
        let now = Utc::now();
        conn.execute(
            "INSERT OR REPLACE INTO ingredients (name, quantity, unit, last_edited) VALUES (?1, ?2, ?3, ?4)",
            &[&name, &quantity.to_string(), &unit, &now.to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn list_ingredients(&self) -> Result<Vec<Ingredient>, Error> {
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare("SELECT name, quantity, unit, last_edited FROM ingredients")?;
        let ingredient_iter = stmt.query_map([], |row| {
            let last_edited_str: String = row.get(3)?;
            Ok(Ingredient {
                name: row.get(0)?,
                quantity: row.get(1)?,
                unit: row.get(2)?,
                last_edited: last_edited_str.parse().unwrap_or_else(|_| Utc::now()),
            })
        })?;

        let mut ingredients = Vec::new();
        for ingredient in ingredient_iter {
            ingredients.push(ingredient?);
        }
        Ok(ingredients)
    }

    // The public update_ingredient method is no longer needed, as add_ingredient handles it.

    pub fn delete_ingredient(&self, name: String) -> Result<(), Error> {
        let conn = self.pool.get()?;
        let rows_affected = conn.execute(
            "DELETE FROM ingredients WHERE name = ?1",
            &[&name],
        )?;

        if rows_affected == 0 {
            Err(Error::NotFound(name))
        } else {
            Ok(())
        }
    }
}
