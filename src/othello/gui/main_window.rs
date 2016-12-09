use gtk::prelude::*;
use gtk;
use gtk::{Window, WindowType, WindowPosition};

use hyper;
use othello::logic::Logic;
use othello::database::DatabaseConnection;

use std::io::prelude::*;
use super::gameboard::GameBoard;

pub struct Gui<T: DatabaseConnection> {
    logic: Logic<T>,
}

impl<T: DatabaseConnection> Gui<T> {
    pub fn new(logic: Logic<T>) -> Gui<T> {
        let window = Window::new(WindowType::Toplevel);
        window.set_title("Othello");
        window.set_border_width(10);
        window.set_position(WindowPosition::Center);
        window.set_default_size(600, 600);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 3);
        window.add(&vbox);

        let gameboard = GameBoard::new();
        vbox.add(&gameboard);

        window.show_all();

        Gui {
            logic: logic,
        }
    }
}
