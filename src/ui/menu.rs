use std::io::{self, Write};

use crate::models::contact::Contact;
use crate::services::contact_service::{
    add_contact, delete_contact, list_contacts, search_contacts, update_contact, toggle_favorite,
};

pub fn run(store: &mut Vec<Contact>) {
    loop {
        print_menu();
        let choice = read_line("Choose an option: ");

        match choice.trim() {
            "1" => handle_add(store),
            "2" => handle_list(store),
            "3" => handle_search(store),
            "4" => handle_update(store),
            "5" => handle_delete(store),
            "6" => handle_toggle_favorite(store),
            "7" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, try again.\n"),
        }
    }
}

fn print_menu() {
    println!("=========================");
    println!("      CONTACT BOOK");
    println!("=========================");
    println!("1. Add Contact");
    println!("2. View Contacts");
    println!("3. Search");
    println!("4. Update");
    println!("5. Delete");
	println!("6. Toggle Favorite");
    println!("7. Exit");
}

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn handle_add(store: &mut Vec<Contact>) {
    let first = read_line("First name: ");
    let last = read_line("Last name: ");
    let phone = read_line("Phone: ");
    let email = read_line("Email: ");

    let contact = Contact::new(&first, &last, &phone, &email);
    match add_contact(store, contact) {
        Ok(()) => println!("Contact added.\n"),
        Err(e) => println!("Error: {}\n", e),
    }
}

fn handle_list(store: &[Contact]) {
    let contacts = list_contacts(store);
    if contacts.is_empty() {
        println!("No contacts yet.\n");
        return;
    }
    for c in contacts {
		let star = if c.favorite { "*" } else { " " };
        println!("{} [{}] {} - {} - {}", star, c.id.unwrap_or(0), c.full_name(), c.phone, c.email);
    }
    println!();
}

fn handle_search(store: &[Contact]) {
    let query = read_line("Search term: ");
    let results = search_contacts(store, &query);
    if results.is_empty() {
        println!("No matches.\n");
        return;
    }
    for c in results {
        println!("[{}] {} - {} - {}", c.id.unwrap_or(0), c.full_name(), c.phone, c.email);
    }
    println!();
}

fn handle_update(store: &mut Vec<Contact>) {
    let id_str = read_line("Contact ID to update: ");
    let id: u32 = match id_str.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid ID.\n");
            return;
        }
    };

    let phone = read_line("New phone (leave blank to skip): ");
    let email = read_line("New email (leave blank to skip): ");

    let phone = if phone.is_empty() { None } else { Some(phone) };
    let email = if email.is_empty() { None } else { Some(email) };

    match update_contact(store, id, phone, email) {
        Ok(()) => println!("Contact updated.\n"),
        Err(e) => println!("Error: {}\n", e),
    }
}

fn handle_delete(store: &mut Vec<Contact>) {
    let id_str = read_line("Contact ID to delete: ");
    let id: u32 = match id_str.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid ID.\n");
            return;
        }
    };

    match delete_contact(store, id) {
        Ok(()) => println!("Contact deleted.\n"),
        Err(e) => println!("Error: {}\n", e),
    }
}

fn handle_toggle_favorite(store: &mut Vec<Contact>) {
	let id_str = read_line("Contact ID to toggle favorite: ");
	let id: u32 = match id_str.parse() {
		Ok(n) => n,
		Err(_) => {
			println!("Invalid ID.\n");
			return;

		}

	};

	match toggle_favorite(store, id) {
		Ok(()) => println!("Favorite status updated.\n"),
		Err(e) => println!("Error: {}\n", e),

	}

}
