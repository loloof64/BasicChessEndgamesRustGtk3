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
        let cells_size = board_clone.size as f64 * 0.111;
        let drawing_area = DrawingArea::new();
        drawing_area.set_size_request(self.size, self.size);
        {
            drawing_area.connect_draw(move |_drawing_area, cx| {
                let pixbuf_white_knight = &board_clone.images.pixbufs[&'N'];
                let pixbuf_black_bishop = &board_clone.images.pixbufs[&'b'];
                Painter::draw_background(cx);
                Painter::draw_piece(cx, &pixbuf_white_knight, cells_size * 3.5, cells_size * 2.5);
                Painter::draw_piece(cx, &pixbuf_black_bishop, cells_size * 8.5, cells_size * 8.5);
                Painter::draw_cells(cx, &board_clone);

                Inhibit(false)
            });
        }

        drawing_area
    }
}


