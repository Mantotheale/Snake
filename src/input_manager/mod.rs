use std::collections::HashMap;
use glium::winit;
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey};

pub struct InputManager {
    keyboard_state: HashMap<KeyCode, ElementState>,
    mouse_buttons_state: HashMap<MouseButton, ElementState>,
    cursor_position: PhysicalPosition<f64>,
    mouse_wheel_delta: f32
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            keyboard_state: HashMap::new(),
            mouse_buttons_state: HashMap::new(),
            cursor_position: PhysicalPosition::new(0f64, 0f64),
            mouse_wheel_delta: 0f32
        }
    }

    pub fn receive_input(&mut self, input: WindowEvent) {
        match input {
            WindowEvent::KeyboardInput {
                event: KeyEvent { physical_key: PhysicalKey::Code(keycode), state, .. }, .. } => {
                self.keyboard_state.insert(keycode, state);
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.mouse_buttons_state.insert(button, state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = position;
            }
            WindowEvent::MouseWheel { delta: MouseScrollDelta::LineDelta(_, delta), .. } => {
                self.mouse_wheel_delta = delta;
            }
            _ => {}
        }
    }

    pub fn key_state(&mut self, key: &KeyCode) -> ElementState {
        self.keyboard_state.get(key).unwrap_or(&ElementState::Released).clone()
    }

    pub fn mouse_button_state(&mut self, mouse: &MouseButton) -> ElementState {
        self.mouse_buttons_state.get(mouse).unwrap_or(&ElementState::Released).clone()
    }

    pub fn cursor_position(&mut self) -> PhysicalPosition<f64> {
        self.cursor_position
    }

    pub fn mouse_wheel_delta(&mut self) -> f32 {
        self.mouse_wheel_delta
    }
}