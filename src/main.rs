extern crate gtk;
#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::Path;

mod database;
mod gui;
mod logic;
pub mod entities;
pub mod errors;
pub mod traits;

use database::Database;
use gui::Gui;
use logic::GameLogic;

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK.");
        return;
    }

    let database = Database::new(Path::new("saved_state.json"));
    let logic = GameLogic::new(database);
    Gui::create(logic);

    gtk::main();
}
