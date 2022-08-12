use gtk::DrawingArea;
use gtk::prelude::*;
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

        let drawing_area = DrawingArea::new();
        drawing_area.connect_draw(|_drawing_area, cx| {
            cx.set_source_rgb(0.3, 0.3, 0.3);
            cx.rectangle(0.0, 0.0, 400.0, 400.0);
            cx.fill().expect("Failed to fill context");
            Inhibit(false)
        });
        win.add(&drawing_area);

        win.show_all();
    });

    app.run();
}
