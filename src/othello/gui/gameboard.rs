use gtk;
use gtk::prelude::*;

const BOARD_SIZE: u32 = 8;

pub struct GameBoard;

impl GameBoard {
    pub fn new() -> gtk::Grid {
        let grid = gtk::Grid::new();
        for _ in 0..BOARD_SIZE {
            grid.insert_row(0);
            grid.insert_column(0);
        }


        grid
    }
}
