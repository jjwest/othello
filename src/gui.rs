use gtk::prelude::*;
use gtk::{Box, Button, DrawingArea, Grid, Menu, MenuBar, MenuItem, Orientation, Overlay, Window,
          WindowPosition, WindowType};
use gtk;

use entities::*;
use traits::*;

use std::cell::RefCell;
use std::rc::Rc;
use std::f64::consts;

const SQUARE_SIZE: i32 = 75;
const BOARD_SIZE: i32 = 8;

pub struct Gui;

impl Gui {
    pub fn new<T: Logic + 'static>(logic: T) {
        let initial_state = Rc::from(RefCell::from(logic.get_initial_state().unwrap()));
        let logic = Rc::from(RefCell::from(logic));

        let window = Window::new(WindowType::Toplevel);
        window.set_title("Othello");
        window.set_border_width(10);
        window.set_position(WindowPosition::Center);
        window.set_size_request(600, 600);
        window.set_resizable(false);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let vbox = Box::new(Orientation::Vertical, 2);
        window.add(&vbox);

        let menu_bar = MenuBar::new();
        let othello = MenuItem::new_with_label("Othello");
        let othello_menu = Menu::new();
        let new_game = MenuItem::new_with_label("Start new game");
        let quit = MenuItem::new_with_label("Quit");
        quit.connect_activate(|_| gtk::main_quit());
        let grid = Grid::new();

        {
            let state = initial_state.clone();
            let logic = logic.clone();
            let grid = grid.clone();
            new_game.connect_activate(move |_| {
                *state.borrow_mut() = logic.borrow_mut().reset_state().unwrap();
                grid.queue_draw();
            });
        }

        othello_menu.append(&new_game);
        othello_menu.append(&quit);
        othello.set_submenu(Some(&othello_menu));
        menu_bar.append(&othello);
        vbox.add(&menu_bar);


        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let background = DrawingArea::new();
                background.set_size_request(SQUARE_SIZE, SQUARE_SIZE);
                let state = initial_state.clone();
                background.connect_draw(move |_, cr| {
                    cr.set_source_rgb(0.0, 0.4, 0.0);
                    cr.rectangle(0.0, 0.0, SQUARE_SIZE as f64, SQUARE_SIZE as f64);
                    cr.fill_preserve();
                    cr.set_source_rgb(0.0, 0.0, 0.0);
                    cr.stroke();

                    if let Some(tile) = state.borrow().board.get(&Point::new(x, y)) {
                        match *tile {
                            Player::Black => cr.set_source_rgb(0.0, 0.0, 0.0),
                            Player::White => cr.set_source_rgb(1.0, 1.0, 1.0),
                        }

                        cr.arc(38.0,
                               38.0,
                               (SQUARE_SIZE / 2 - 10) as f64,
                               0.,
                               2. * consts::PI);
                        cr.fill();
                    }

                    Inhibit(false)
                });

                let button = Button::new();
                button.set_size_request(50, 50);
                button.set_opacity(0.0);

                let overlay = Overlay::new();
                overlay.add(&background);
                overlay.add_overlay(&button);
                grid.attach(&overlay, x, y, 1, 1);

                let logic = logic.clone();
                let state = initial_state.clone();
                let grid = grid.clone();
                button.connect_clicked(move |_| {
                    let mut logic = logic.borrow_mut();
                    if let Ok(new_state) = logic.place_tile(Point::new(x, y)) {
                        *state.borrow_mut() = new_state;
                        grid.queue_draw();
                    }
                });
            }
        }

        vbox.add(&grid);
        window.show_all();
    }
}
