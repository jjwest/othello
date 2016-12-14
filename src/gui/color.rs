use gtk::prelude::*;
use gtk;

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color {
            r: r  as f64 / 255.0,
            g: g  as f64 / 255.0,
            b: b  as f64 / 255.0,
        }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}
