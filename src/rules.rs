use super::entities::*;

pub trait Rule {
    fn is_valid(&self, placement: &Point, state: &GameStateEntity) -> bool;
}

pub struct RuleBook {
    rules: Vec<Box<Rule>>,
}

impl RuleBook {
    pub fn new() -> RuleBook {
        RuleBook { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: Box<Rule>) {
        self.rules.push(rule);
    }

    pub fn placement_allowed(&self, pos: &Point, state: &GameStateEntity) -> bool {
        self.rules.iter().all(|rule| rule.is_valid(pos, state))
    }
}

pub struct MustExistAdjacentEnemy;

impl Rule for MustExistAdjacentEnemy {
    fn is_valid(&self, placement: &Point, state: &GameStateEntity) -> bool {
        for y in placement.y - 1..placement.y + 2 {
            for x in placement.x - 1..placement.x + 2 {
                if let Some(neighbour) = state.board.get(&Point::new(x, y)) {
                    if *neighbour != state.active_player {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub struct MustExistConnectedFriendly;

impl Rule for MustExistConnectedFriendly {
    fn is_valid(&self, placement: &Point, state: &GameStateEntity) -> bool {
        for x in placement.x - 1..placement.x + 2 {
            for y in placement.y - 1..placement.y + 2 {
                if let Some(neighbour) = state.board.get(&Point::new(x, y)) {
                    if *neighbour != state.active_player {
                        let delta_x = placement.x - x;
                        let delta_y = placement.y - y;
                        let distance = 2;

                        while let Some(next) = state.board.get(&Point::new(delta_x * distance, delta_y * distance)) {
                            if *next == state.active_player {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}
