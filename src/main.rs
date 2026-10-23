mod app;

use app::App;

fn main() {
    let mut app = App::new(640, 480, "Windah");
    app.init();
    app.main_loop();
}
