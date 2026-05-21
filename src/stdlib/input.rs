use std::collections::HashSet;
use winit::keyboard::KeyCode;
use winit::event::ElementState;

pub struct InputManager {
    current_keys: HashSet<KeyCode>,
    previous_keys: HashSet<KeyCode>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            current_keys: HashSet::new(),
            previous_keys: HashSet::new(),
        }
    }

    /// Should be called at the start of every frame
    pub fn tick(&mut self) {
        self.previous_keys = self.current_keys.clone();
    }

    pub fn update_key(&mut self, key: KeyCode, state: ElementState) {
        if state == ElementState::Pressed {
            self.current_keys.insert(key);
        } else {
            self.current_keys.remove(&key);
        }
    }

    /// Returns true if the key is currently held down
    pub fn is_down(&self, key: KeyCode) -> bool {
        self.current_keys.contains(&key)
    }

    /// Returns true only on the frame the key was first pressed
    pub fn was_pressed(&self, key: KeyCode) -> bool {
        self.current_keys.contains(&key) && !self.previous_keys.contains(&key)
    }

    /// Returns true only on the frame the key was released
    pub fn was_released(&self, key: KeyCode) -> bool {
        !self.current_keys.contains(&key) && self.previous_keys.contains(&key)
    }

    /// Provided for backward compatibility with current game updates
    pub fn keys(&self) -> &HashSet<KeyCode> {
        &self.current_keys
    }
}
