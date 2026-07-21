mod database;
mod errors;
mod models;
mod services;
mod ui;

use models::contact::Contact;
use database::connection::establish_connection;
use database::schema::run_migrations;

fn main() {

	let conn = establish_connection().expect("Failed to connect to database");
	run_migrations(&conn).expect("Failed to run migrations");
	println!("Database ready at data/contacts.db");


    //let mut store: Vec<Contact> = Vec::new();
    //ui::menu::run(&mut store);
}
