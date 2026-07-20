#[derive(Debug, Clone)]
pub struct Contact {
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: String,
    pub favorite: bool,
    pub category_id: Option<u32>,
}

impl Contact {
    pub fn new(first_name: &str, last_name: &str, phone: &str, email: &str) -> Self {
        Contact {
            id: None, // assigned by the database later
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            phone: phone.to_string(),
            email: email.to_string(),
            favorite: false,
            category_id: None,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}