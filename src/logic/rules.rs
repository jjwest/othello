use entities::*;
use traits::*;

pub struct MustExistAdjacentEnemy;

impl Rule for MustExistAdjacentEnemy {
    fn is_valid(&self, placement: &Point, state: &GameStateEntity) -> bool {
        for y in placement.y - 1..placement.y + 2 {
            for x in placement.x - 1..placement.x + 2 {
                if let Some(neighbour) = state.board.get(&Point::new(x, y)) {
                    if *neighbour != state.active_player {
                        return true;
                    }
                }
            }
        }

        false
    }
}

pub struct MustExistConnectedFriendly;

impl Rule for MustExistConnectedFriendly {
    fn is_valid(&self, pos: &Point, state: &GameStateEntity) -> bool {
        let neighbours = vec![
            Point::new(pos.x -1, pos.y -1),   Point::new(pos.x, pos.y - 1), Point::new(pos.x + 1, pos.y -1),
            Point::new(pos.x - 1, pos.y),                                   Point::new(pos.x + 1, pos.y),
            Point::new(pos.x - 1, pos.y + 1), Point::new(pos.x, pos.y + 1), Point::new(pos.x + 1, pos.y + 1)
        ];

        for neighbour in neighbours {
            if let Some(tile) = state.board.get(&Point::new(neighbour.x, neighbour.y)) {
                if *tile != state.active_player {
                    let delta_x = neighbour.x - pos.x;
                    let delta_y = neighbour.y - pos.y;
                    let mut distance = 2;
                    let mut next_pos = Point::new(pos.x + delta_x * distance, pos.y + delta_y * distance);

                    while let Some(next) = state.board.get(&next_pos) {
                        if *next == state.active_player {
                            return true;
                        } else {
                            distance += 1;
                            next_pos = Point::new(pos.x + delta_x * distance, pos.y + delta_y * distance);
                        }
                    }
                }
            }
        }

        false
    }
}
