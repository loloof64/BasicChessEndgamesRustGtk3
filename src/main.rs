use gdk_pixbuf::{InterpType, Pixbuf};
use gio::MemoryInputStream;
use glib::Bytes;
use gtk::prelude::*;
use gtk::DrawingArea;
use gtk::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.loloof64.BasicChessEndgames")
        .build();

    app.connect_activate(|app| {
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(400)
            .default_height(400)
            .title("Basic Chess Endgames")
            .build();

        let image_data = include_bytes!("./resources/Chess_blt45.svg");
        let image_data = Bytes::from(image_data);
        let image_stream = MemoryInputStream::from_bytes(&image_data);
        let pixbuf = Pixbuf::from_stream(&image_stream, None::<&gio::Cancellable>)
            .expect("Failed to create stream for image.");
        let pixbuf = pixbuf
            .scale_simple(140, 140, InterpType::Bilinear)
            .expect("Failed to resize the image.");
        let drawing_area = DrawingArea::new();
        drawing_area.connect_draw(move |_drawing_area, cx| {
            cx.set_source_rgb(0.3, 0.3, 0.3);
            cx.rectangle(0.0, 0.0, 400.0, 400.0);
            cx.fill().expect("Failed to fill context");

            cx.set_source_pixbuf(&pixbuf, 0.0, 0.0);
            cx.paint().expect("Failed to draw image.");

            Inhibit(false)
        });
        win.add(&drawing_area);

        win.show_all();
    });

    app.run();
}
