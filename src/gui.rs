use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition, Orientation};
use gtk;

use traits::*;
use entities::*;

use std::cell::RefCell;
use std::rc::Rc;

const SQUARE_SIZE: i32 = 50;
const BOARD_SIZE: i32 = 8;

pub struct Gui<T: Logic> {
    logic: Rc<RefCell<T>>,
    curr_state: Rc<RefCell<GameStateEntity>>,
}

impl<T: Logic + 'static> Gui<T> {
    pub fn new(logic: T) -> Gui<T> {
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

        let vbox = gtk::Box::new(Orientation::Vertical, 2);
        window.add(&vbox);

        let menu_bar = gtk::MenuBar::new();
        let othello = gtk::MenuItem::new_with_label("Othello");
        let othello_menu = gtk::Menu::new();
        let new_game = gtk::MenuItem::new_with_label("Start new game");
        let quit = gtk::MenuItem::new_with_label("Quit");

        quit.connect_activate(|_| gtk::main_quit());


        othello_menu.append(&new_game);
        othello_menu.append(&quit);
        othello.set_submenu(Some(&othello_menu));
        menu_bar.append(&othello);
        vbox.add(&menu_bar);

        let grid = gtk::Grid::new();

        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let background = gtk::DrawingArea::new();
                background.set_size_request(SQUARE_SIZE, SQUARE_SIZE);
                let state = initial_state.clone();
                background.connect_draw(move |_, cr| {
                    cr.set_source_rgb(0.2, 0.4, 0.8);
                    cr.rectangle(0.0, 0.0, SQUARE_SIZE as f64, SQUARE_SIZE as f64);
                    cr.fill_preserve();
                    cr.set_source_rgb(0.0, 0.0, 0.0);
                    cr.stroke_preserve();

                    if let Some(tile) = state.borrow().board.get(&Point::new(x, y)) {
                        match *tile {
                            Player::Black => {
                                cr.set_source_rgb(0.0, 0.0, 0.0);
                                cr.fill();
                            },
                            Player::White => {
                                cr.set_source_rgb(1.0, 1.0, 1.0);
                                cr.fill();
                            },
                        }
                    } else {
                        cr.new_path();
                    }

                    gtk::Inhibit(false)
                });

                let button = gtk::Button::new();
                button.set_size_request(50, 50);
                button.set_opacity(0.0);

                let overlay = gtk::Overlay::new();
                overlay.add(&background);
                overlay.add_overlay(&button);
                grid.attach(&overlay, x, y, 1, 1);

                let logic = logic.clone();
                let state = initial_state.clone();
                button.connect_clicked(move |_| {
                    let mut logic = logic.borrow_mut();
                    if let Ok(new_state) = logic.place_tile(Point::new(x, y)) {
                        *state.borrow_mut() = new_state;
                    }
                });
            }
        }

        vbox.add(&grid);
        window.show_all();

        Gui {
            logic: logic,
            curr_state: initial_state,
        }

    }
}
