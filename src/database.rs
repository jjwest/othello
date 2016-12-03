use super::entities::{Point, Player, GameStateEntity};

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use errors::OthelloResult;

use serde_json;

pub trait DatabaseConnection {
    fn save_state(&mut self, state: GameStateEntity) -> OthelloResult<()>;
    fn load_state(&self) -> OthelloResult<GameStateEntity>;
}

pub struct Database<'a> {
    location: &'a Path,
}

impl<'a> Database<'a> {
    pub fn new(location: &Path) -> Database {
        Database {
            location: location,
        }
    }
}

impl<'a> DatabaseConnection for Database<'a> {
    fn save_state(&mut self, state: GameStateEntity) -> OthelloResult<()> {
        let serializable = SerializableState::from(state);
        let serialized = serde_json::to_string(&serializable)?;

        let mut file = File::create(self.location)?;
        write!(&mut file, "{}", serialized)?;

        Ok(())
    }

    fn load_state(&self) -> OthelloResult<GameStateEntity> {
        if let Ok(mut file) = File::open(self.location) {
            let mut serialized = String::new();
            file.read_to_string(&mut serialized)?;
            let deserialized = serde_json::from_str(&serialized)?;

            Ok(SerializableState::into_state(deserialized))
        } else {
            let mut board = HashMap::new();
            board.insert(Point::new(3, 3), Player::Black);
            board.insert(Point::new(4, 4), Player::Black);
            board.insert(Point::new(3, 4), Player::White);
            board.insert(Point::new(4, 3), Player::White);

            Ok(GameStateEntity::new(board, Player::Black, None))
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializableState {
    active_player: Player,
    winner: Option<Player>,
    board: Vec<(Point, Player)>,
}

impl SerializableState {
    fn from(state: GameStateEntity) -> SerializableState {
        SerializableState {
            active_player: state.active_player,
            winner: state.winner,
            board: state.board.into_iter().collect(),
        }
    }

    fn into_state(state: SerializableState) -> GameStateEntity {
        GameStateEntity::new(
            state.board.into_iter().collect(),
            state.active_player,
            state.winner
        )
    }
}
