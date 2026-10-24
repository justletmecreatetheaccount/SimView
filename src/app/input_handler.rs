pub struct InputHandler {
    // Config
    pan_modifier: glfw::Modifiers,
    orbit_button: glfw::MouseButton,

    // State
    previous_cursor_position: (f64, f64),
    dragged_button: Option<glfw::MouseButton>, // Some -> currently being dragged (option is technically useless but clearer)
    active_modifiers: glfw::Modifiers,
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

            previous_cursor_position: (0.0, 0.0),
            dragged_button: None,
            active_modifiers: glfw::Modifiers::empty(),
        }
    }

    pub fn handle_input(&mut self, event: &glfw::WindowEvent) -> Option<AppActions> {
        match event {
            glfw::WindowEvent::Key(key, _, action, modifier) => match (key, action, modifier) {
                (glfw::Key::Escape, glfw::Action::Press, _) => Some(AppActions::Exit),
                _ => None,
            },
            glfw::WindowEvent::MouseButton(button, action, modifier) => {
                match (button, action, modifier) {
                    (glfw::MouseButtonLeft, glfw::Action::Press, m) => {
                        self.dragged_button = Some(*button);
                        self.active_modifiers = self.active_modifiers & *m;
                        None
                    }
                    (glfw::MouseButtonLeft, glfw::Action::Release, _) => {
                        self.dragged_button = None;
                        self.active_modifiers = glfw::Modifiers::empty();
                        None
                    }
                    _ => None,
                }
            }
            glfw::WindowEvent::CursorPos(x, y) => {
                if let Some(button) = self.dragged_button {
                    match button {
                        glfw::MouseButtonLeft => {
                            let (dx, dy) = (
                                x - self.previous_cursor_position.0,
                                y - self.previous_cursor_position.1,
                            );
                            self.previous_cursor_position = (*x, *y);
                            Some(AppActions::OrbitCamera {
                                dx: dx as f32,
                                dy: dy as f32,
                            })
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
