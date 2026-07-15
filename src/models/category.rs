#[derive(Debug, Clone)]
pub struct Category {

	pub id: Option<u32>,
	pub name: String,

}

impl Category {

	pub fn new(name: &str) -> Self {

		Category {

			id: None, // assigned by the database(sqlite) on insert, same as Contact
			name: name.to_string(),

		}

	}

}
