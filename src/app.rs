mod gpu;
mod renderer;
mod window;

use glfw::{Action, Key};
use gpu::Gpu;
use renderer::Renderer;
use window::Window;

pub struct App {
    glfw_instance: glfw::Glfw,
    window: Window,
    renderer: Renderer,
    gpu: Gpu,
}

impl App {
    pub fn new(width: u32, height: u32, window_title: &str) -> App {
        use glfw::fail_on_errors;
        let mut glfw_instance = glfw::init(fail_on_errors!()).unwrap();
        glfw_instance.window_hint(glfw::WindowHint::Resizable(false));
        glfw_instance.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let window = Window::new(width, height, window_title, &mut glfw_instance);

        App {
            glfw_instance,
            window,
            renderer: Renderer {},
            gpu: Gpu {},
        }
    }

    pub fn init(&mut self) {}

    // Loop until the user closes the window
    pub fn main_loop(&mut self) {
        while !self.window.should_close() {
            // Poll for and process events
            self.glfw_instance.poll_events();
            for (_, event) in self.window.drain_events() {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        self.window.set_should_close(true);
                    }
                    _ => {}
                }
            }
        }
    }
}
