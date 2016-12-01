use super::entities::{Point, Color, GameStateEntity};

use std::collections::HashMap;
use std::io;

use errors::*;

use serde_json;

pub trait DatabaseConnection {
    fn save_state(&mut self, state: GameStateEntity) -> OthelloResult<()>;
    fn load_state(&self) -> OthelloResult<GameStateEntity>;
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
    fn save_state(&mut self, state: GameStateEntity) -> OthelloResult<()> {
        let board = state.board.into_iter().collect::<Vec<_>>();
        let serialized_board = serde_json::to_string(&board)?;
        let serialized_player = serde_json::to_string(&state.active_player)?;
        let serialized_player = serde_json::to_string(&state.active_player)?;
        Ok(())
    }

    fn load_state(&self) -> OthelloResult<GameStateEntity> {
        Ok(self.state.clone())
    }
}
