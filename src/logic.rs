use super::rules::*;
use super::DatabaseConnection;
use super::entities::*;

use std::io;

pub struct Logic<T: DatabaseConnection> {
    rules: RuleBook,
    database: T,
}

impl<T: DatabaseConnection> Logic<T> {
    pub fn new(rules: RuleBook, database: T) -> Logic<T> {
        Logic {
            rules: rules,
            database: database,
        }
    }

    pub fn place_tile(&mut self, position: Point) -> Result<GameStateEntity, io::Error> {
        let mut state = self.database.load_state()?;

        if self.rules.placement_allowed(&position, &state) {
            state.board.insert(position.clone(), state.active_player.clone());
            convert_tiles(&position, &mut state);
            state.active_player = match state.active_player {
                Color::Black => Color::White,
                Color::White => Color::Black,
            };
            self.database.save_state(state)?;
            self.database.load_state()
        } else {
            Ok(state)
        }
    }
}

fn convert_tiles(pos: &Point, state: &mut GameStateEntity) {
    let neighbours = vec![
        Point::new(pos.x -1, pos.y -1),   Point::new(pos.x, pos.y - 1), Point::new(pos.x + 1, pos.y -1),
        Point::new(pos.x - 1, pos.y),                                   Point::new(pos.x + 1, pos.y),
        Point::new(pos.x - 1, pos.y + 1), Point::new(pos.x, pos.y + 1), Point::new(pos.x + 1, pos.y + 1)
    ];

    for neigbour in neighbours {
        if let Some(tile) = state.board.get(&neigbour) {
            if *tile != state.active_player {
                let delta_x = pos.x - neigbour.x;
                let delta_y = pos.y - neigbour.y;

            }
        }
    }
}
