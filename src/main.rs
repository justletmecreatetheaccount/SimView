mod app;

use app::App;
use glfw::{Action, Context, Key};

fn main() {
    let mut app = App::new(640, 480, "Windah");
    app.init();
    app.main_loop();
}
