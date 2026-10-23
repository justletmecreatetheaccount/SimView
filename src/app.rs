mod gpu;
mod input_handler;
mod renderer;
mod window;

use gpu::Gpu;
use renderer::Renderer;
use window::WindowManager;

use crate::app::input_handler::{AppActions, InputHandler};

pub struct App {
    glfw_instance: glfw::Glfw,
    window: WindowManager,
    renderer: Renderer,
    gpu: Gpu,
    input_handler: InputHandler,
}

impl App {
    pub fn new(width: u32, height: u32, window_title: &str) -> App {
        use glfw::fail_on_errors;
        let mut glfw_instance = glfw::init(fail_on_errors!()).unwrap();
        glfw_instance.window_hint(glfw::WindowHint::Resizable(false));
        glfw_instance.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let window = WindowManager::new(width, height, window_title, &mut glfw_instance);

        let gpu = Gpu::new(&window);

        let renderer = Renderer::new(&gpu);

        let input_handler = InputHandler::new();

        App {
            glfw_instance,
            window,
            renderer,
            gpu,
            input_handler,
        }
    }

    pub fn init(&mut self) {}

    // Loop until the user closes the window
    pub fn main_loop(&mut self) {
        while !self.window.should_close() {
            self.renderer.render(&self.gpu);
            // Poll for and process events
            self.glfw_instance.poll_events();
            for (_, event) in self.window.drain_events() {
                println!("{:?}", event);

                if let Some(action) = self.input_handler.handle_input(&event) {
                    match action {
                        AppActions::Exit => {
                            self.window.set_should_close(true);
                        }
                        AppActions::OrbitCamera { dx, dy } => {}
                        _ => {}
                    }
                }

                self.input_handler.reset_state();
            }
        }
    }
}
