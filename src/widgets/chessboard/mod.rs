use gtk::prelude::*;
use gtk::DrawingArea;

mod pieces_images;
use pieces_images::PiecesImages;

mod painter;
use painter::Painter;

#[derive(Clone)]
pub struct ChessBoard {
    size: i32,
    images: PiecesImages,
}

impl ChessBoard {
    pub fn new(size: i32) -> Self {
        let images = PiecesImages::new(size);
        Self { size, images }
    }

    pub fn view(&self) -> DrawingArea {
        let board_clone = self.clone();
        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(self.size, self.size);
        {
            drawing_area.connect_draw(move |_drawing_area, cx| {
                let pixbuf = &board_clone.images.pixbufs[&'N'];
                Painter::draw_background(cx);
                Painter::draw_piece(cx, &pixbuf);
                Painter::draw_cells(cx, &board_clone);

                Inhibit(false)
            });
        }

        drawing_area
    }
}


