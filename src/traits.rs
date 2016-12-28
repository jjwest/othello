use errors::*;
use entities::*;

pub trait Rule {
    fn is_valid(&self, placement: Point, state: &GameStateEntity) -> bool;
}

pub trait DatabaseConnection {
    fn load_state(&self) -> Result<GameStateEntity>;
    fn reset_state(&mut self) -> Result<GameStateEntity>;
    fn save_state(&mut self, state: GameStateEntity) -> Result<()>;
}

pub trait Logic {
    fn get_initial_state(&self) -> Result<GameStateEntity>;
    fn place_tile(&mut self, position: Point) -> Result<GameStateEntity>;
    fn reset_state(&mut self) -> Result<GameStateEntity>;
}
