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
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    // pressed
    // inputs actively being pressed

    #[inline]
    pub fn pressed(&self, code: T) -> bool {
        self.pressed.contains(&code)
    }

    #[inline]
    pub fn iter_pressed(&self) -> impl Iterator<Item = &T> {
        self.pressed.iter()
    }

    #[inline]
    pub fn get_pressed(&self) -> &FxHashSet<T> {
        &self.pressed
    }

    // just pressed
    // inputs pressed in the last event 'burst'

    #[inline]
    pub fn just_pressed(&self, code: T) -> bool {
        self.just_pressed.contains(&code)
    }

    #[inline]
    pub fn iter_just_pressed(&self) -> impl Iterator<Item = &T> {
        self.just_pressed.iter()
    }

    #[inline]
    pub fn get_just_pressed(&self) -> &FxHashSet<T> {
        &self.just_pressed
    }

    // just released
    // inputs released in the last event 'burst'

    #[inline]
    pub fn just_released(&self, code: T) -> bool {
        self.just_released.contains(&code)
    }

    #[inline]
    pub fn iter_just_released(&self) -> impl Iterator<Item = &T> {
        self.just_released.iter()
    }

    #[inline]
    pub fn get_just_released(&self) -> &FxHashSet<T> {
        &self.just_released
    }

    //

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.pressed.is_empty() && self.just_pressed.is_empty() && self.just_released.is_empty()
    }

    #[inline]
    pub fn clear(&mut self) {
        self.just_pressed.clear();
        self.just_released.clear();
    }

    pub(crate) fn event(&mut self, state: ElementState, code: T) {
        // self.clear();
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
