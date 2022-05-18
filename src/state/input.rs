use rustc_hash::FxHashSet;
use std::hash::Hash;
use winit::event::ElementState;

//

#[derive(Debug, Clone)]
pub struct Input<T> {
    pressed: FxHashSet<T>,
    just_pressed: FxHashSet<T>,
    just_released: FxHashSet<T>,
}

//

impl<T> Default for Input<T>
where
    T: Copy + Eq + Hash,
{
    fn default() -> Self {
        Self {
            pressed: Default::default(),
            just_pressed: Default::default(),
            just_released: Default::default(),
        }
    }
}

impl<T> Input<T>
where
    T: Copy + Eq + Hash,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pressed(&self, code: T) -> bool {
        self.pressed.contains(&code)
    }

    pub fn just_pressed(&self, code: T) -> bool {
        self.just_pressed.contains(&code)
    }

    pub fn just_released(&self, code: T) -> bool {
        self.just_released.contains(&code)
    }

    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub(crate) fn event(&mut self, state: ElementState, code: T) {
        self.clear();
        match state {
            ElementState::Pressed => {
                self.pressed.insert(code);
                self.just_pressed.insert(code);
            }
            ElementState::Released => {
                self.pressed.remove(&code);
                self.just_released.insert(code);
            }
        }
    }
}
