#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod database;
pub mod logic;
pub mod rules;
pub mod entities;
pub mod errors;

#[cfg(test)]
mod tests;

use database::Database;
use entities::*;
use logic::Logic;
use rules::*;
use std::path::Path;

fn main() {
    let rules = load_rules();
    let database = Database::new(&Path::new("database.json"));
    let mut logic = Logic::new(rules, database);
    let state = logic.place_tile(Point::new(4, 5)).unwrap();

    for (pos, color) in &state.board {
        println!("{:#?}: {:#?}", color, pos);
    }
}

fn load_rules() -> RuleBook {
    let mut rules = RuleBook::new();
    rules.add_rule(Box::new(MustExistAdjacentEnemy));
    rules.add_rule(Box::new(MustExistConnectedFriendly));

    rules
}
