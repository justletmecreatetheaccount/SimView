use glfw::{GlfwReceiver, PWindow, WindowEvent};

pub struct WindowManager {
    glfw_window: Box<PWindow>,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    window_size: (u32, u32),
    _window_title: String,
}

impl WindowManager {
    pub fn new(
        width: u32,
        height: u32,
        window_title: &str,
        glfw_instance: &mut glfw::Glfw,
    ) -> WindowManager {
        let (mut glfw_window, events) = glfw_instance
            .create_window(width, height, window_title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw_window.set_key_polling(true);

        let glfw_window = Box::new(glfw_window);

        WindowManager {
            window_size: (width, height),
            _window_title: String::from(window_title),
            glfw_window,
            events,
        }
    }

    pub fn should_close(&self) -> bool {
        self.glfw_window.should_close()
    }

    pub fn drain_events(&self) -> Vec<(f64, WindowEvent)> {
        glfw::flush_messages(&self.events).collect()
    }

    pub fn set_should_close(&mut self, value: bool) {
        self.glfw_window.set_should_close(value);
    }

    pub fn get_glfw_window(&self) -> &PWindow {
        &self.glfw_window
    }

    pub fn get_window_size(&self) -> (u32, u32) {
        self.window_size
    }
}
