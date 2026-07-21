use rusqlite::Connection;
use crate::errors::AppError;

pub fn run_migrations(conn: &Connection) -> Result<(), AppError> {
	conn.execute (
		"CREATE TABLE IF NOT EXISTS categories (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			name TEXT NOT NULL UNIQUE

		)",

		(),

	

	)
	.map_err(|e| AppError::DatabaseError(e.to_string()))?;

	conn.execute(
		"CREATE TABLE IF NOT EXISTS contacts (
			id INTEGER PRIMARY KEY AUTOINCREMENT,
			first_name TEXT NOT NULL,
			last_name TEXT NOT NULL,
			phone TEXT NOT NULL,
			email TEXT NOT NULL,
			favorite INTEGER NOT NULL DEFAULT 0,
			category_id INTEGER,
			FOREIGN KEY (category_id) REFERENCES categories(id)

		)",
		(),

	)

	.map_err(|e| AppError::DatabaseError(e.to_string()))?;

	Ok(())

}
