use std::collections::HashMap;

use cairo::Context;
use gdk_pixbuf::Pixbuf;
use gio::MemoryInputStream;
use glib::Bytes;
use gtk::prelude::*;
use gtk::DrawingArea;

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

#[derive(Clone)]
struct PiecesImages {
    streams: HashMap<char, MemoryInputStream>,
    pixbufs: HashMap<char, Pixbuf>,
}

impl PiecesImages {
    pub fn new(size: i32) -> Self {
        let cells_size = (size as f64 * 0.111).ceil() as i32;
        let streams = PiecesImages::build_streams();
        let pixbufs = PiecesImages::build_pixbufs(&streams, cells_size);

        Self { streams, pixbufs }
    }

    fn build_streams() -> HashMap<char, MemoryInputStream> {
        let mut result = HashMap::new();
        let pieces_types = vec!['P', 'N', 'B', 'R', 'Q', 'K', 'p', 'n', 'b', 'r', 'q', 'k'];
        let svg_defs: Vec<&[u8]> = vec![
            include_bytes!("./vectors/Chess_plt45.svg"),
            include_bytes!("./vectors/Chess_nlt45.svg"),
            include_bytes!("./vectors/Chess_blt45.svg"),
            include_bytes!("./vectors/Chess_rlt45.svg"),
            include_bytes!("./vectors/Chess_qlt45.svg"),
            include_bytes!("./vectors/Chess_klt45.svg"),
            include_bytes!("./vectors/Chess_pdt45.svg"),
            include_bytes!("./vectors/Chess_ndt45.svg"),
            include_bytes!("./vectors/Chess_bdt45.svg"),
            include_bytes!("./vectors/Chess_rdt45.svg"),
            include_bytes!("./vectors/Chess_qdt45.svg"),
            include_bytes!("./vectors/Chess_kdt45.svg"),
        ];
        let pieces_refs: Vec<(_, _)> = pieces_types.iter().zip(svg_defs.iter()).collect();

        for (kind, data) in pieces_refs.iter() {
            let kind = **kind;
            let image_data = **data;

            let image_data = Bytes::from(image_data);
            let image_stream = MemoryInputStream::from_bytes(&image_data);

            result.insert(kind, image_stream);
        }

        result
    }

    fn build_pixbufs(
        streams: &HashMap<char, MemoryInputStream>,
        size: i32,
    ) -> HashMap<char, Pixbuf> {
        let mut result = HashMap::new();

        for kind in streams.keys() {
            let image_stream = &streams[kind];

            let pixbuf = Pixbuf::from_stream_at_scale(
                image_stream,
                size,
                size,
                true,
                None::<&gio::Cancellable>,
            )
            .expect("Failed to interpret image.");

            result.insert(*kind, pixbuf);
        }

        result
    }
}
