extern crate gtk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::path::Path;

#[macro_use]
pub mod macros;

mod database;
mod gui;
mod logic;
pub mod entities;
pub mod errors;
pub mod traits;

use database::Database;
use gui::Gui;
use logic::*;

fn main() {
    if gtk::init().is_err() {
        errln!("Failed to initialize GTK.");
        return;
    }

    let database = Database::new(Path::new("saved_state.json"));
    let logic = GameLogic::new(database);
    Gui::create(logic);

    gtk::main();
}
