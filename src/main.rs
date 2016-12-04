#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate gtk;

mod othello;
use othello::*;
use gtk::prelude::*;

use std::path::Path;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }


    let rules = load_rules();
    let database = Database::new(&Path::new("database.json"));
    let logic = Logic::new(rules, database);
    let gui = Gui::new(logic);

    gtk::main();
}

fn load_rules() -> RuleBook {
    let mut rules = RuleBook::new();
    rules.add_rule(Box::new(MustExistAdjacentEnemy));
    rules.add_rule(Box::new(MustExistConnectedFriendly));

    rules
}
