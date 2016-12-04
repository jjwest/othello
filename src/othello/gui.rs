use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition, main_quit};

use othello::logic::Logic;
use othello::database::DatabaseConnection;

pub struct Gui<T: DatabaseConnection> {
    window: Window,
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
            main_quit();
            Inhibit(false)
        });
        window.show_all();

        Gui {
            window: window,
            logic: logic,
        }
    }

}
