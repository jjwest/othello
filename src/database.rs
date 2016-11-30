use super::entities::{Point, Color, GameStateEntity};

use std::collections::HashMap;
use std::io;

pub trait DatabaseConnection {
    fn save_state(&mut self, state: GameStateEntity) -> io::Result<()>;
    fn load_state(&self) -> io::Result<GameStateEntity>;
}

pub struct Database {
    state: GameStateEntity,
}

impl Database {
    pub fn new() -> Database {
        let mut board = HashMap::new();
        board.insert(Point::new(3, 3), Color::Black);
        board.insert(Point::new(4, 4), Color::Black);
        board.insert(Point::new(4, 3), Color::White);
        board.insert(Point::new(3, 4), Color::White);

        Database {
            state: GameStateEntity::new(board, Color::Black, None),
        }
    }
}

impl DatabaseConnection for Database {
    fn save_state(&mut self, state: GameStateEntity) -> io::Result<()> {
        self.state = state;
        Ok(())
    }

    fn load_state(&self) -> io::Result<GameStateEntity> {
        Ok(self.state.clone())
    }
}
