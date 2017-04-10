use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Player {
    Black,
    White,
}

pub type GameBoard = HashMap<Point, Player>;

#[derive(Clone, Debug)]
pub struct GameStateEntity {
    pub board: GameBoard,
    pub active_player: Player,
    pub winner: Option<Player>,
}

impl GameStateEntity {
    pub fn new(board: GameBoard, active_player: Player, winner: Option<Player>) -> GameStateEntity {
        GameStateEntity {
            board,
            active_player,
            winner,
        }
    }
}
