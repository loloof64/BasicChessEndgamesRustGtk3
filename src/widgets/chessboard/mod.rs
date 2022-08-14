use cairo::Context;
use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;
use gtk::prelude::*;
use gtk::DrawingArea;

#[derive(Clone)]
pub struct ChessBoard {
    size: i32,
}

impl ChessBoard {
    pub fn new(size: i32) -> Self {
        Self { size }
    }

    pub fn view(&self) -> DrawingArea {
        let image_data = include_bytes!("./vectors/Chess_blt45.svg");
        let image_data = Bytes::from(image_data);
        let image_stream = MemoryInputStream::from_bytes(&image_data);
        let pixbuf =
            Pixbuf::from_stream_at_scale(&image_stream, 140, 140, true, None::<&gio::Cancellable>)
                .expect("Failed to interpret image.");

        let board_clone = self.clone();
        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(self.size, self.size);
        {
            drawing_area.connect_draw(move |_drawing_area, cx| {
                Painter::draw_background(cx);
                Painter::draw_piece(cx, &pixbuf);
                Painter::draw_cells(cx, &board_clone);

                Inhibit(false)
            });
        }

        drawing_area
    }
}

struct Painter {}

impl Painter {
    fn color_int_to_float(color_int: i32) -> f64 {
        color_int as f64 / 1.0
    }

    fn draw_background(cx: &Context) {
        cx.set_source_rgb(0.3, 0.3, 0.8);
        cx.rectangle(0.0, 0.0, 400.0, 400.0);
        cx.fill().unwrap();
    }

    fn draw_piece(cx: &Context, pixbuf: &Pixbuf) {
        cx.set_source_pixbuf(&pixbuf, 30.0, 50.0);
        cx.paint().expect("Failed to draw image.");
    }

    fn draw_cells(cx: &Context, board: &ChessBoard) {
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
