mod errors;
mod services;
mod models;

use models::category::Category;
use models::contact::Contact;
use services::contact_service::validate_contact;

fn main() {


	let c = Contact::new("Ada", "Lovelace", "553-3454", "ada@example.com");
	let cat = Category::new("Work");


	println!("Created contact: {} ({})", c.full_name(), c.phone);
	println!("Created Category: {}", cat.name);

	let existing: Vec<Contact> = vec![
		Contact::new("Ada", "Lovelace", "555-1234", "ada@email.com"),
	
	];

	let new_contact = Contact::new("Grace", "Hopper", "444-3321", "grace@example.com");
	match validate_contact(&new_contact, &existing) {
		Ok(()) => println!("Valid contact: {}", new_contact.full_name()),
		Err(e) => println!("Rejected: {}", e),
	}

	let duplicate = Contact::new("Fake", "Ada", "555-1234", "diff@ex.com");
	match validate_contact(&duplicate, &existing) {
		Ok(()) => println!("Valid contact: {}", duplicate.full_name()),
		Err(e) => println!("Rejected: {}", e),

	}

	let bad_email = Contact::new("No", "Email", "555-0000", "not-an-email");
	match validate_contact(&bad_email, &existing) {
		Ok(()) => println!("Valid contact: {}", bad_email.full_name()),
		Err(e) => println!("Rejected: {}", e),

	}

}
