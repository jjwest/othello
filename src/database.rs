use super::entities::{Point, Color, GameStateEntity};

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use errors::*;

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
        let mut file = File::open(self.location)?;
        let mut serialized = String::new();
        file.read_to_string(&mut serialized)?;
        let deserialized = serde_json::from_str(&serialized)?;

        Ok(SerializableState::into_state(deserialized))
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableState {
    active_player: Color,
    winner: Option<Color>,
    board: Vec<(Point, Color)>,
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
