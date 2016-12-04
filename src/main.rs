mod othello;
use othello::*;

use std::path::Path;

fn main() {
    let rules = load_rules();
    let database = Database::new(&Path::new("database.json"));
    let mut logic = Logic::new(rules, database);
    let _ = logic.place_tile(Point::new(5, 3)).unwrap();
    let state = logic.place_tile(Point::new(3, 2)).unwrap();

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
