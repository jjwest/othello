#![feature(proc_macro)]

extern crate gtk;
extern crate hyper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod othello;
use othello::*;

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
