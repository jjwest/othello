#![feature(proc_macro)]

extern crate glib;
extern crate gtk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod database;
mod gui;
mod logic;
pub mod entities;
pub mod errors;
pub mod traits;

use database::Database;
use gui::Gui;
use logic::*;
use std::path::Path;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let rules = load_rules();
    let database = Database::new(&Path::new("database.json"));
    let logic = GameLogic::new(rules, database);
    let gui = Gui::new(logic);

    gtk::main();
}

fn load_rules() -> RuleBook {
    let mut rules = RuleBook::new();
    rules.add_rule(Box::new(MustExistAdjacentEnemy));
    rules.add_rule(Box::new(MustExistConnectedFriendly));

    rules
}
