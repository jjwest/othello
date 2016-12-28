use entities::*;
use errors::*;
use traits::*;

use super::RuleBook;

pub struct GameLogic<T: DatabaseConnection> {
    rules: RuleBook,
    database: T,
}

impl<T> Logic for GameLogic<T>
    where T: DatabaseConnection
{
    fn place_tile(&mut self, position: Point) -> Result<GameStateEntity> {
        let mut state = self.database.load_state()?;

        if self.rules.placement_allowed(position, &state) {
            state.board.insert(position, state.active_player);
            convert_tiles(position, &mut state);
            state.active_player = match state.active_player {
                Player::Black => Player::White,
                Player::White => Player::Black,
            };
            self.database.save_state(state)?;
            self.database.load_state()
        } else {
            Ok(state)
        }
    }

    fn get_initial_state(&self) -> Result<GameStateEntity> {
        self.database.load_state()
    }

    fn reset_state(&mut self) -> Result<GameStateEntity> {
        self.database.reset_state()
    }
}

impl<D: DatabaseConnection> GameLogic<D> {
    pub fn new(rules: RuleBook, database: D) -> GameLogic<D> {
        GameLogic {
            rules: rules,
            database: database,
        }
    }
}

fn convert_tiles(pos: Point, state: &mut GameStateEntity) {
    let neighbours = vec![Point::new(pos.x - 1, pos.y - 1),
                          Point::new(pos.x, pos.y - 1),
                          Point::new(pos.x + 1, pos.y - 1),
                          Point::new(pos.x - 1, pos.y),
                          Point::new(pos.x + 1, pos.y),
                          Point::new(pos.x - 1, pos.y + 1),
                          Point::new(pos.x, pos.y + 1),
                          Point::new(pos.x + 1, pos.y + 1)];

    let mut tiles_to_swap = Vec::new();

    for neighbour in neighbours {
        if let Some(neighbour_tile) = state.board.get(&neighbour) {
            if *neighbour_tile != state.active_player {
                let delta_x = neighbour.x - pos.x;
                let delta_y = neighbour.y - pos.y;
                let mut distance = 2;
                let mut next_pos = Point::new(pos.x + delta_x * distance,
                                              pos.y + delta_y * distance);
                let mut middle_tiles = vec![neighbour];

                while let Some(next_tile) = state.board.get(&next_pos) {
                    if *next_tile == *neighbour_tile {
                        middle_tiles.push(next_pos);
                        distance += 1;
                        next_pos = Point::new(pos.x + delta_x * distance,
                                              pos.y + delta_y * distance);
                    } else {
                        tiles_to_swap.append(&mut middle_tiles);
                        break;
                    }
                }
            }
        }
    }

    for tile in tiles_to_swap {
        state.board.insert(tile, state.active_player);
    }
}
