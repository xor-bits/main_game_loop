use super::input::Input;
use crate::event::Event;
use std::ops::{Deref, DerefMut};
use winit::event::{KeyboardInput, VirtualKeyCode, WindowEvent};

//

#[derive(Debug, Clone, Default)]
pub struct KeyboardState {
    inner: Input<VirtualKeyCode>,
    auto_clear: bool,
}

//

impl KeyboardState {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn with_auto_clear(mut self) -> Self {
        self.set_auto_clear(true);
        self
    }

    #[inline]
    pub fn set_auto_clear(&mut self, auto_clear: bool) {
        self.auto_clear = auto_clear;
    }

    pub fn event(&mut self, event: &Event) {
        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state,
                                virtual_keycode: Some(virtual_keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                self.inner.event(*state, *virtual_keycode);
            }
            Event::RedrawEventsCleared if self.auto_clear => self.inner.clear(),
            _ => (),
        }
    }
}

impl Deref for KeyboardState {
    type Target = Input<VirtualKeyCode>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for KeyboardState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
