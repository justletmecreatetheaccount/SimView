use std::collections::HashMap;

pub struct InputHandler {
    // Config
    pan_modifier: glfw::Modifiers,
    key_bindings: HashMap<(glfw::Key, glfw::Modifiers), InputKeyActions>,
    mouse_bindings: HashMap<(glfw::MouseButton, glfw::Modifiers), InputMouseActions>,
    scroll_bindings: HashMap<ScrollTypes, InputScrollActions>,

    // State
    previous_cursor_position: (f64, f64),
    mouse_action: Option<InputMouseActions>, // Some -> currently being dragged (option is technically useless but clearer)
}

pub(crate) enum AppActions {
    Exit,
    OrbitCamera { dx: f32, dy: f32 },
    PanCamera { dx: f32, dy: f32 },
    ZoomCamera { dy: f32 },
}

#[derive(Clone, Copy)]
enum InputKeyActions {
    Exit,
}

#[derive(Clone, Copy)]
enum InputMouseActions {
    OrbitCamera,
    PanCamera,
}

#[derive(Clone, Copy)]
enum InputScrollActions {
    ZoomCamera,
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum ScrollTypes {
    Vertical,
    Horizontal,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            pan_modifier: glfw::Modifiers::Shift,

            // Can eventually be set with a config file
            key_bindings: HashMap::from([(
                (glfw::Key::Escape, glfw::Modifiers::empty()),
                InputKeyActions::Exit,
            )]),
            mouse_bindings: HashMap::from([
                (
                    (glfw::MouseButton::Button1, glfw::Modifiers::empty()),
                    InputMouseActions::OrbitCamera,
                ),
                (
                    (glfw::MouseButton::Button1, glfw::Modifiers::Shift),
                    InputMouseActions::PanCamera,
                ),
            ]),
            scroll_bindings: HashMap::from([(
                ScrollTypes::Vertical,
                InputScrollActions::ZoomCamera,
            )]),

            previous_cursor_position: (0.0, 0.0),
            mouse_action: None,
        }
    }

    pub fn handle_input(&mut self, event: &glfw::WindowEvent) -> Option<AppActions> {
        match event {
            glfw::WindowEvent::Key(key, _, action, modifier) => match action {
                glfw::Action::Press => {
                    if let Some(action) = self.key_bindings.get(&(*key, *modifier)) {
                        match action {
                            InputKeyActions::Exit => Some(AppActions::Exit),
                        }
                    } else {
                        None
                    }
                }
                glfw::Action::Release => {
                    if let Some(action) = self.key_bindings.get(&(*key, *modifier)) {
                        match action {
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                glfw::Action::Repeat => None,
            },

            glfw::WindowEvent::MouseButton(button, action, modifier) => match action {
                glfw::Action::Press => {
                    if let Some(action) = self.mouse_bindings.get(&(*button, *modifier)) {
                        self.mouse_action = Some(*action);
                        None
                    } else {
                        None
                    }
                }
                glfw::Action::Release => {
                    self.mouse_action = None;
                    None
                }
                glfw::Action::Repeat => None,
            },

            glfw::WindowEvent::CursorPos(x, y) => {
                if let Some(mouse_action) = self.mouse_action {
                    match mouse_action {
                        InputMouseActions::OrbitCamera => {
                            let (dx, dy) = (
                                x - self.previous_cursor_position.0,
                                y - self.previous_cursor_position.1,
                            );
                            // Update continuously to avoid jumps
                            self.previous_cursor_position = (*x, *y);
                            Some(AppActions::OrbitCamera {
                                dx: dx as f32,
                                dy: dy as f32,
                            })
                        }
                        InputMouseActions::PanCamera => {
                            let (dx, dy) = (
                                x - self.previous_cursor_position.0,
                                y - self.previous_cursor_position.1,
                            );
                            // Update continuously to avoid jumps
                            self.previous_cursor_position = (*x, *y);
                            Some(AppActions::PanCamera {
                                dx: dx as f32,
                                dy: dy as f32,
                            })
                        }
                    }
                } else {
                    // Update continuously to avoid jumps
                    self.previous_cursor_position = (*x, *y);
                    None
                }
            }

            glfw::WindowEvent::Scroll(_dx, dy) => {
                if let Some(action) = self.scroll_bindings.get(&ScrollTypes::Vertical) {
                    match action {
                        InputScrollActions::ZoomCamera => {
                            Some(AppActions::ZoomCamera { dy: (*dy) as f32 })
                        }
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
