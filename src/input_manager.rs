// input_manager.rs

use std::collections::HashMap;

use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Escape,
    Tab,
    Ctrl,
    Alt,
    Enter,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Space,
}

pub struct KeyEvent {
    pub key_down: bool,
    pub key_up: bool,
}

impl KeyEvent {
    pub fn new(key_down: bool, key_up: bool) -> Self {
        KeyEvent { key_down, key_up }
    }
}

pub struct InputManager {
    keys: HashMap<KeyCode, KeyEvent>,
    mouse_position: Vec2,
    left_mouse_button_down: bool,
    right_mouse_button_down: bool,
    left_mouse_button_up: bool,
    right_mouse_button_up: bool,
}

impl InputManager {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        // Initialize keys (can be simplified with a macro if needed)
        keys.insert(KeyCode::A, KeyEvent::new(false, false));
        keys.insert(KeyCode::B, KeyEvent::new(false, false));
        keys.insert(KeyCode::C, KeyEvent::new(false, false));
        keys.insert(KeyCode::D, KeyEvent::new(false, false));
        keys.insert(KeyCode::E, KeyEvent::new(false, false));
        keys.insert(KeyCode::F, KeyEvent::new(false, false));
        keys.insert(KeyCode::G, KeyEvent::new(false, false));
        keys.insert(KeyCode::H, KeyEvent::new(false, false));
        keys.insert(KeyCode::I, KeyEvent::new(false, false));
        keys.insert(KeyCode::J, KeyEvent::new(false, false));
        keys.insert(KeyCode::K, KeyEvent::new(false, false));
        keys.insert(KeyCode::L, KeyEvent::new(false, false));
        keys.insert(KeyCode::M, KeyEvent::new(false, false));
        keys.insert(KeyCode::N, KeyEvent::new(false, false));
        keys.insert(KeyCode::O, KeyEvent::new(false, false));
        keys.insert(KeyCode::P, KeyEvent::new(false, false));
        keys.insert(KeyCode::Q, KeyEvent::new(false, false));
        keys.insert(KeyCode::R, KeyEvent::new(false, false));
        keys.insert(KeyCode::S, KeyEvent::new(false, false));
        keys.insert(KeyCode::T, KeyEvent::new(false, false));
        keys.insert(KeyCode::U, KeyEvent::new(false, false));
        keys.insert(KeyCode::V, KeyEvent::new(false, false));
        keys.insert(KeyCode::W, KeyEvent::new(false, false));
        keys.insert(KeyCode::X, KeyEvent::new(false, false));
        keys.insert(KeyCode::Y, KeyEvent::new(false, false));
        keys.insert(KeyCode::Z, KeyEvent::new(false, false));
        keys.insert(KeyCode::Escape, KeyEvent::new(false, false));
        keys.insert(KeyCode::Tab, KeyEvent::new(false, false));
        keys.insert(KeyCode::Ctrl, KeyEvent::new(false, false));
        keys.insert(KeyCode::Alt, KeyEvent::new(false, false));
        keys.insert(KeyCode::Enter, KeyEvent::new(false, false));
        keys.insert(KeyCode::Zero, KeyEvent::new(false, false));
        keys.insert(KeyCode::One, KeyEvent::new(false, false));
        keys.insert(KeyCode::Two, KeyEvent::new(false, false));
        keys.insert(KeyCode::Three, KeyEvent::new(false, false));
        keys.insert(KeyCode::Four, KeyEvent::new(false, false));
        keys.insert(KeyCode::Five, KeyEvent::new(false, false));
        keys.insert(KeyCode::Six, KeyEvent::new(false, false));
        keys.insert(KeyCode::Seven, KeyEvent::new(false, false));
        keys.insert(KeyCode::Eight, KeyEvent::new(false, false));
        keys.insert(KeyCode::Nine, KeyEvent::new(false, false));
        keys.insert(KeyCode::Space, KeyEvent::new(false, false));

        InputManager {
            keys,
            mouse_position: Vec2::new(0.0, 0.0),
            left_mouse_button_down: false,
            right_mouse_button_down: false,
            left_mouse_button_up: false,
            right_mouse_button_up: false,
        }
    }

    pub fn reset_key_states(&mut self) {
        self.set_mouse_button_up(0, false);
        self.set_mouse_button_up(1, false);

        for (_, key) in self.keys.iter_mut() {
            key.key_up = false;
        }
    }

    pub fn set_key(&mut self, key_code: KeyCode, value: bool) {
        if let Some(key) = self.keys.get_mut(&key_code) {
            key.key_down = value;
            if !value {
                key.key_up = true;
            }
        } else {
            println!("The key couldn't be found!!!");
        }
    }

    pub fn get_key_down(&self, key_code: KeyCode) -> bool {
        match self.keys.get(&key_code) {
            Some(key) => key.key_down,
            None => {
                println!("The key couldn't be found!!!");
                false
            }
        }
    }

    pub fn get_key_up(&self, key_code: KeyCode) -> bool {
        match self.keys.get(&key_code) {
            Some(key) => key.key_up,
            None => {
                println!("The key couldn't be found!!!");
                false
            }
        }
    }

    pub fn set_mouse_position(&mut self, mouse_pos: Vec2) {
        self.mouse_position = mouse_pos;
    }

    pub fn get_mouse_position(&self) -> Vec2 {
        self.mouse_position
    }

    pub fn set_mouse_button_down(&mut self, button: i32, value: bool) {
        match button {
            0 => self.left_mouse_button_down = value,
            1 => self.right_mouse_button_down = value,
            _ => (),
        }
    }

    pub fn set_mouse_button_up(&mut self, button: i32, value: bool) {
        match button {
            0 => self.left_mouse_button_up = value,
            1 => self.right_mouse_button_up = value,
            _ => (),
        }
    }

    pub fn get_mouse_button_down(&self, button: i32) -> bool {
        match button {
            0 => self.left_mouse_button_down,
            1 => self.right_mouse_button_down,
            _ => false,
        }
    }

    pub fn get_mouse_button_up(&self, button: i32) -> bool {
        match button {
            0 => self.left_mouse_button_up,
            1 => self.right_mouse_button_up,
            _ => false,
        }
    }
}
