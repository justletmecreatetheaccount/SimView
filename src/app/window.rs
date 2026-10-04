use glfw::{GlfwReceiver, WindowEvent};

pub struct Window {
    glfw_window: glfw::PWindow,
    events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    window_size: (u32, u32),
    window_title: String,
}

impl Window {
    pub fn new(
        width: u32,
        height: u32,
        window_title: &str,
        glfw_instance: &mut glfw::Glfw,
    ) -> Window {
        let (mut glfw_window, events) = glfw_instance
            .create_window(width, height, window_title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        glfw_window.set_key_polling(true);

        Window {
            window_size: (width, height),
            window_title: String::from(window_title),
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
}
