use entities::*;
use traits::Rule;

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
