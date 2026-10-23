pub struct InputHandler {
    // Config
    pan_modifier: glfw::Modifiers,
    orbit_button: glfw::MouseButton,

    // State
    dragged_button: Option<glfw::MouseButton>, // Some currently being dragged
}

pub(crate) enum AppActions {
    Exit,
    OrbitCamera { dx: f32, dy: f32 },
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            pan_modifier: glfw::Modifiers::Shift,
            orbit_button: glfw::MouseButton::Button1,
            dragged_button: None,
        }
    }

    pub fn handle_input(&self, event: &glfw::WindowEvent) -> Option<AppActions> {
        match event {
            glfw::WindowEvent::Key(key, _, action, modifier) => match key {
                glfw::Key::Escape => Some(AppActions::Exit),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn reset_state(&mut self) {
        self.dragged_button = None;
    }
}
