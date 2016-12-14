use glib;
use gtk::prelude::*;
use gtk::{Window, WindowType, WindowPosition};
use gtk;

use std::rc::Rc;
use traits::*;
use entities::*;

use super::color::Color;

const SQUARE_SIZE: f64 = 50.0;

pub struct MainWindow<T: Logic> {
    logic: T,
    window: gtk::Window,
    vbox: gtk::Box,
    menu_bar: gtk::MenuBar,
    drawing_area: gtk::DrawingArea,
}

impl<T: Logic> MainWindow<T> {
    pub fn new(logic: T) -> MainWindow<T> {
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

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
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

        let state = logic.get_initial_state().unwrap();
        let drawing_area = gtk::DrawingArea::new();
        drawing_area.set_hexpand(true);
        drawing_area.set_vexpand(true);

        drawing_area.connect_draw(move |_, cr| {
            let bg = Color::from_rgb(0, 153, 51);
            for y in 0..8 {
                for x in 0..8 {
                    cr.rectangle(x as f64 * SQUARE_SIZE,
                                 y as f64 * SQUARE_SIZE,
                                 SQUARE_SIZE,
                                 SQUARE_SIZE);

                    // Draw background color
                    cr.set_source_rgb(bg.r(), bg.g(), bg.b());
                    cr.fill_preserve();

                    // Draw borders
                    cr.set_source_rgb(0.0, 0.0, 0.0);
                    cr.stroke_preserve();

                    // Draw player colors
                    if let Some(tile) = state.board.get(&Point::new(x, y)) {
                        match *tile {
                            Player::Black => {
                                cr.set_source_rgb(0.0, 0.0, 0.0);
                                cr.fill();
                            },
                            Player::White => {
                                cr.set_source_rgb(1.0, 1.0, 1.0);
                                cr.fill();
                            }
                        }
                    } else {
                        cr.new_path();
                    }
                }
            }
            gtk::Inhibit(false)
        });

        vbox.add(&drawing_area);
        window.show_all();

        MainWindow {
            logic: logic,
            window: window,
            vbox: vbox,
            menu_bar: menu_bar,
            drawing_area: drawing_area,
        }

    }
}
