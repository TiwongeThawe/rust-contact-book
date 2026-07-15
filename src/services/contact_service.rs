use crate::errors::AppError;
use crate::models::contact::Contact;

pub fn validate_contact(contact: &Contact, existing: &[Contact]) -> Result<(), AppError> {
	if contact.first_name.trim().is_empty() || contact.last_name.trim().is_empty() {
		return Err(AppError::EmptyName);
	
	}

	if !is_valid_email(&contact.email) {
		return Err(AppError::InvalidEmail);

	}

	if !is_valid_phone(&contact.phone) {
		return Err(AppError::InvalidPhone);

	}

	if is_duplicate(contact, existing) {
		return Err(AppError::DuplicateContact);

	}

	Ok(())
}

fn is_valid_email(email: &str) -> bool {
	// Simple check for now: something@something.something
	// Not RFC-compliant, but good enough for a CLI app.
	let parts: Vec<&str> = email.split('@').collect();
	parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.')

}

fn is_valid_phone(phone: &str) -> bool {
	//Require at least 7 digits characters, allow spaces/dashes/parens
	let digit_count = phone.chars().filter(|c| c.is_ascii_digit()).count();
	digit_count >= 7

}

fn is_duplicate(contact: &Contact, existing: &[Contact]) -> bool {
	existing
		.iter()
		.any(|c| c.phone == contact.phone || c.email == contact.email)

}
