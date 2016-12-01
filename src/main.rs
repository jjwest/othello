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

pub use database::*;
pub use entities::*;
pub use rules::*;

use logic::Logic;

fn main() {
    let rules = load_rules();
    let database = Database::new();
    let mut logic = Logic::new(rules, database);
    let state = logic.place_tile(Point::new(3, 5)).unwrap();

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
