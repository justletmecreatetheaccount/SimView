mod camera;
mod gpu;
mod input_handler;
mod renderer;
mod window;

use gpu::Gpu;
use renderer::Renderer;
use window::WindowManager;

use crate::app::{
    camera::Camera,
    input_handler::{AppActions, InputHandler},
};

pub struct App {
    input_handler: InputHandler,
    camera: Camera,
    renderer: Renderer,
    gpu: Gpu,
    window: WindowManager,
    glfw_instance: glfw::Glfw,
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

        let aspect_ratio = window.framebuffer_size().0 as f32 / window.framebuffer_size().1 as f32;
        let camera = Camera::new(aspect_ratio);

        let input_handler = InputHandler::new();

        App {
            glfw_instance,
            window,
            gpu,
            renderer,
            camera,
            input_handler,
        }
    }

    pub fn init(&mut self) {}

    // Loop until the user closes the window
    pub fn main_loop(&mut self) {
        while !self.window.should_close() {
            self.renderer.render(&self.gpu, &self.camera);
            // Poll for and process events
            self.glfw_instance.poll_events();
            for (_, event) in self.window.drain_events() {
                println!("event : {:?}", event);
                if let Some(action) = self.input_handler.handle_input(&event) {
                    match action {
                        AppActions::Exit => {
                            self.window.set_should_close(true);
                        }
                        AppActions::OrbitCamera { dx, dy } => self.camera.orbit(dx, dy),
                        AppActions::PanCamera { dx, dy } => self.camera.pan(dx, dy),
                        AppActions::ZoomCamera { dy } => self.camera.zoom(dy),
                    }
                }
            }
        }
    }
}
