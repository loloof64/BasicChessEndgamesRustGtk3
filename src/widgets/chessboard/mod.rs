use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;
use gtk::prelude::*;
use gtk::DrawingArea;

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

        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(self.size, self.size);
        drawing_area.connect_draw(move |_drawing_area, cx| {
            // clearing background
            cx.set_source_rgb(0.3, 0.3, 0.8);
            cx.rectangle(0.0, 0.0, 400.0, 400.0);
            cx.fill().unwrap_or_default();

            // draw a single chess piece
            cx.set_source_pixbuf(&pixbuf, 30.0, 50.0);
            cx.paint().expect("Failed to draw image.");

            Inhibit(false)
        });

        drawing_area
    }
}
