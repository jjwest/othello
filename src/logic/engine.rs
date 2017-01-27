use entities::*;
use errors::*;
use traits::*;

use super::RuleBook;

pub struct GameLogic<D: DatabaseConnection> {
    rules: RuleBook,
    database: D,
}

impl<D> Logic for GameLogic<D>
    where D: DatabaseConnection
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
            self.database.save_state(state.clone())?;
            Ok(state)
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

fn convert_tiles(origin: Point, state: &mut GameStateEntity) {
    let neighbours = vec![Point::new(origin.x - 1, origin.y - 1),
                          Point::new(origin.x, origin.y - 1),
                          Point::new(origin.x + 1, origin.y - 1),
                          Point::new(origin.x - 1, origin.y),
                          Point::new(origin.x + 1, origin.y),
                          Point::new(origin.x - 1, origin.y + 1),
                          Point::new(origin.x, origin.y + 1),
                          Point::new(origin.x + 1, origin.y + 1)];

    let opponent_color = match state.active_player {
        Player::Black => Player::White,
        Player::White => Player::Black,
    };

    let mut tiles_to_convert = Vec::new();

    for neighbour in neighbours {
        if let Some(neighbour_color) = state.board.get(&neighbour) {
            if *neighbour_color == opponent_color {
                let delta_x = neighbour.x - origin.x;
                let delta_y = neighbour.y - origin.y;
                let mut distance = 2;
                let mut next_pos = Point::new(origin.x + delta_x * distance,
                                              origin.y + delta_y * distance);
                let mut middle_tiles = vec![neighbour];

                while let Some(next_tile) = state.board.get(&next_pos) {
                    if *next_tile == opponent_color {
                        middle_tiles.push(next_pos);
                        distance += 1;
                        next_pos = Point::new(origin.x + delta_x * distance,
                                              origin.y + delta_y * distance);
                    } else {
                        tiles_to_convert.append(&mut middle_tiles);
                        break;
                    }
                }
            }
        }
    }

    for tile in tiles_to_convert {
        state.board.insert(tile, state.active_player);
    }
}
