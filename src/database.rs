use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use errors::Result;
use serde_json;
use super::entities::{Point, Player, GameStateEntity};
use traits::DatabaseConnection;


#[derive(Debug)]
pub struct Database<'a> {
    location: &'a Path,
}

impl<'a> Database<'a> {
    pub fn new(location: &Path) -> Database {
        Database { location }
    }

    fn get_default_state() -> GameStateEntity {
        let mut board = HashMap::new();
        board.insert(Point::new(3, 3), Player::Black);
        board.insert(Point::new(4, 4), Player::Black);
        board.insert(Point::new(3, 4), Player::White);
        board.insert(Point::new(4, 3), Player::White);

        GameStateEntity::new(board, Player::Black, None)
    }
}

impl<'a> DatabaseConnection for Database<'a> {
    fn save_state(&mut self, state: GameStateEntity) -> Result<()> {
        let serializable = SerializableState::from(state);
        let serialized = serde_json::to_string_pretty(&serializable)?;

        let mut file = File::create(self.location)?;
        write!(&mut file, "{}", serialized)?;

        Ok(())
    }

    fn load_state(&self) -> Result<GameStateEntity> {
        if let Ok(mut file) = File::open(self.location) {
            let mut serialized = String::new();
            file.read_to_string(&mut serialized)?;
            let deserialized: SerializableState = serde_json::from_str(&serialized)?;

            Ok(deserialized.into())
        } else {
            Ok(Database::get_default_state())
        }
    }

    fn reset_state(&mut self) -> Result<GameStateEntity> {
        self.save_state(Database::get_default_state())?;
        self.load_state()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SerializableState {
    active_player: Player,
    winner: Option<Player>,
    board: Vec<(Point, Player)>,
}

impl Into<GameStateEntity> for SerializableState {
    fn into(self) -> GameStateEntity {
        GameStateEntity::new(
            self.board.into_iter().collect(),
            self.active_player,
            self.winner,
        )
    }
}

impl From<GameStateEntity> for SerializableState {
    fn from(state: GameStateEntity) -> SerializableState {
        SerializableState {
            active_player: state.active_player,
            winner: state.winner,
            board: state.board.into_iter().collect(),
        }
    }
}
