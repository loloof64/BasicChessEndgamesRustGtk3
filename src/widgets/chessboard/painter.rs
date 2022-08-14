use cairo::Context;
use gdk_pixbuf::Pixbuf;
use gtk::prelude::*;

use super::ChessBoard;
pub struct Painter {}

impl Painter {
    fn color_int_to_float(color_int: i32) -> f64 {
        color_int as f64 / 1.0
    }

    pub fn draw_background(cx: &Context) {
        cx.set_source_rgb(0.3, 0.3, 0.8);
        cx.rectangle(0.0, 0.0, 400.0, 400.0);
        cx.fill().unwrap();
    }

    pub fn draw_piece(cx: &Context, pixbuf: &Pixbuf) {
        cx.set_source_pixbuf(&pixbuf, 30.0, 50.0);
        cx.paint().expect("Failed to draw image.");
    }

    pub fn draw_cells(cx: &Context, board: &ChessBoard) {
        let cells_size = board.size as f64 * 0.111f64;
        cx.set_source_rgb(
            Painter::color_int_to_float(255),
            Painter::color_int_to_float(222),
            Painter::color_int_to_float(273),
        );
        cx.rectangle(10.0, 20.0, cells_size, cells_size);
        cx.fill().unwrap();
    }
}