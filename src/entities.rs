use std::collections::HashMap;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

#[derive(Clone, PartialEq)]
pub enum Color {
    Black,
    White,
}

pub type GameBoard = HashMap<Point, Color>;

#[derive(Clone)]
pub struct GameStateEntity {
    pub board: GameBoard,
    pub active_player: Color,
    pub winner: Option<Color>,
}

impl GameStateEntity {
    pub fn new(board: GameBoard, active_player: Color, winner: Option<Color>) -> GameStateEntity {
        GameStateEntity {
            board: board,
            active_player: active_player,
            winner: winner,
        }
    }
}
