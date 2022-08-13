use gtk::prelude::{ApplicationExt, ApplicationExtManual, ContainerExt, WidgetExt};
use gtk::{Application, ApplicationWindow};

mod widgets;
use widgets::chessboard::ChessBoard;

fn build_ui(app: &Application) {
    let win = ApplicationWindow::builder()
            .application(app)
            .default_width(400)
            .default_height(400)
            .title("Basic Chess Endgames")
            .build();

        let chessboard = ChessBoard::new(300);
        win.add(&chessboard.view());

        win.show_all();
}

fn main() {
    let app = Application::builder()
        .application_id("com.loloof64.BasicChessEndgames")
        .build();

    app.connect_activate(|app| {
        build_ui(app);
    });

    app.run();
}
