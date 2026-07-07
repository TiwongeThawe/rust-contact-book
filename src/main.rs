#[derive(Debug)]

struct Contact {
	name: String,
	phone: String,
	email: String,
}

fn find_contact<'a>(

	contacts: &'a Vec<Contact>,
	name: &str,

) -> Option<&'a Contact> {

	for person in contacts.iter() {

		if person.name == name {

			return Some(person);

		}

	}

	None

}

fn update_phone(contacts: &mut Vec<Contact>, name: &str, new_phone: &str) {

	for person in contacts.iter_mut() {

		if person.name == name {

			person.phone = String::from(new_phone);
			break;

		}

	}

}

fn main() {

	let mut contacts = vec![

		Contact {

			name: String::from("Alice"),
			phone: String::from("0978645311"),
			email: String::from("alice@gmail.com"),

		},

		Contact {

			name: String::from("Bob"),
			phone: String::from("0960748478"),
			email: String::from("bab@fmail.com"),

		},

		Contact {

			name: String::from("Charlie"),
			phone: String::from("0767334621"),
			email: String::from("charlie@whomail.com"),

		},

	];

	if let Some(person) = find_contact(&contacts, "Bob") {

		println!("Before update: {:?}", person);

	}

	update_phone(&mut contacts, "Bob", "0966554433");

	if let Some(person) = find_contact(&contacts, "Bob") {

		println!("After update: {:?}", person);

	}

}










