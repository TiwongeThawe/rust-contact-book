mod errors;
mod models;
mod services;
mod ui;

use models::contact::Contact;

fn main() {
    let mut store: Vec<Contact> = Vec::new();
    ui::menu::run(&mut store);
}