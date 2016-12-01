use rules::*;
use database::DatabaseConnection;
use entities::*;
use errors::*;

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

    pub fn place_tile(&mut self, position: Point) -> Result<GameStateEntity, OthelloError> {
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

    let mut tiles_to_swap = Vec::new();

    for neighbour in neighbours {
        if let Some(neighbour_tile) = state.board.get(&neighbour) {
            if *neighbour_tile != state.active_player {
                let delta_x = neighbour.x - pos.x;
                let delta_y = neighbour.y - pos.y;
                let mut distance = 1;
                let mut next_pos = Point::new(pos.x + delta_x * distance, pos.y + delta_y * distance);
                let mut middle_tiles = Vec::new();

                while let Some(next_tile) = state.board.get(&next_pos) {
                    if *next_tile == *neighbour_tile {
                        middle_tiles.push(next_pos);
                        distance += 1;
                        next_pos = Point::new(pos.x + delta_x * distance, pos.y + delta_y * distance);
                    } else {
                        tiles_to_swap.append(&mut middle_tiles);
                        break;
                    }
                }
            }
        }
    }

    for tile in tiles_to_swap {
        state.board.insert(tile, state.active_player.clone());
    }
}
