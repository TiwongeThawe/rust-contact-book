use std::fmt;

#[derive(Debug)]

pub enum AppError {

	EmptyName,
	InvalidEmail,
	InvalidPhone,
	DuplicateContact,
	NotFound,

}

impl fmt::Display for AppError {

	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

		match self {
			AppError::EmptyName => write!(f, "Name cannot be empty"),
			AppError::InvalidEmail => write!(f, "Email format is invalid"),
			AppError::InvalidPhone => write!(f, "Phone format is invalid"),
			AppError::DuplicateContact => write!(f, "A contact with this phone or email already exists"),
			AppError::NotFound => write!(f, "Contact not found"),
		}

	}

}

impl std::error::Error for AppError {}
