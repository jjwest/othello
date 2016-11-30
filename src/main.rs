pub mod database;
pub mod logic;
pub mod rules;
pub mod entities;

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

    let new_state = logic.
}

fn load_rules() -> RuleBook {
    let mut rules = RuleBook::new();
    rules.add_rule(Box::new(MustExistAdjacentEnemy));
    rules.add_rule(Box::new(MustExistConnectedFriendly));

    rules
}
