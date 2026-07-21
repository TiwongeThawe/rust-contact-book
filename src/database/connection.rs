use rusqlite::Connection;
use crate::errors::AppError;

pub fn establish_connection() -> Result<Connection, AppError> {
	std::fs::create_dir_all("data").map_err(|e| AppError::DatabaseError(e.to_string()))?;

	let conn = Connection::open("data/contacts.db")
		.map_err(|e| AppError::DatabaseError(e.to_string()))?;

	Ok(conn)

}
