mod app;

use app::App;

fn main() {
    let mut app = App::new(1280, 720, "Windah");
    app.init();
    app.main_loop();
}
