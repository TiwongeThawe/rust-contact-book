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
	let parts: Vec<&str> = email.split('@').collect();
	parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.')

}

fn is_valid_phone(phone: &str) -> bool {
	let digit_count = phone.chars().filter(|c| c.is_ascii_digit()).count();
	digit_count >= 7
}

fn is_duplicate(contact: &Contact, existing: &[Contact]) -> bool {
	existing.iter().any(|c| c.phone == contact.phone || c.email == contact.email)
}

// ----CRUD-----


pub fn add_contact(store: &mut Vec<Contact>, mut contact: Contact) -> Result<(), AppError> {
	validate_contact(&contact, store)?;
	let next_id = store.iter().filter_map(|c| c.id).max().unwrap_or(0) + 1;
	contact.id = Some(next_id);
	store.push(contact);
	Ok(())
}


pub fn list_contacts(store: &[Contact]) -> &[Contact] {
	store

}

pub fn find_contacts<'a>(store: &'a [Contact], id: u32) -> Result<&'a Contact, AppError> {
	store.iter().find(|c| c.id == Some(id)).ok_or(AppError::NotFound)

}

pub fn update_contact(
	store: &mut [Contact],
	id: u32,
	phone: Option<String>,
	email: Option<String>,
) -> Result<(), AppError> {
	let contact = store
		.iter_mut()
		.find(|c| c.id == Some(id))
		.ok_or(AppError::NotFound)?;

	if let Some(p) = phone {
		contact.phone = p;

	}
	Ok(())

}

pub fn delete_contact(store: &mut Vec<Contact>, id: u32) -> Result<(), AppError> {
	let len_before = store.len();
	store.retain(|c| c.id == Some(id));
	if store.len() == len_before {
		return Err(AppError::NotFound);

	}

	Ok(())

}

pub fn search_contacts<'a>(store: &'a [Contact], query: &str) -> Vec<&'a Contact> {
	let q = query.to_lowercase();
	store
		.iter()
		.filter(|c| {
			c.full_name().to_lowercase().contains(&q)
				|| c.phone.contains(&q)
				|| c.email.to_lowercase().contains(&q)
			})
			.collect()

}
