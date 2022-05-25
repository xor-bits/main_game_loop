use super::input::Input;
use crate::event::{CustomEvent, Event};
use gilrs::EventType;
use glam::Vec2;
use rustc_hash::FxHashMap;
use std::ops::{Deref, DerefMut};
use winit::event::ElementState;

//

pub use gilrs::{Axis as GamepadAxis, Button as GamepadButton, GamepadId as Gamepad};

//

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GamepadButtonInput {
    pub gamepad: Gamepad,
    pub button: GamepadButton,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct GamepadAxisInput {
    pub gamepad: Gamepad,
    pub axis: GamepadAxis,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DeadZone {
    pub left_stick: Vec2,
    pub right_stick: Vec2,
    pub left: f32,
    pub right: f32,
    pub d_pad: Vec2,
}

#[derive(Debug, Clone, Default)]
pub struct GamepadState {
    inner: Input<GamepadButtonInput>,
    axes: FxHashMap<GamepadAxisInput, f32>,
    gamepads: FxHashMap<Gamepad, DeadZone>,
    auto_clear: bool,
}

//

impl Default for DeadZone {
    fn default() -> Self {
        Self {
            left_stick: Vec2::new(0.04, 0.04),
            right_stick: Vec2::new(0.04, 0.04),
            left: f32::EPSILON,
            right: f32::EPSILON,
            d_pad: Vec2::new(f32::EPSILON, f32::EPSILON),
        }
    }
}

impl GamepadState {
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
            // gamepad events
            Event::UserEvent(CustomEvent::GamePadEvent(gilrs::Event { id, event, .. })) => {
                let gamepad = *id;
                let deadzone = self.gamepads.entry(gamepad).or_default();
                match *event {
                    EventType::ButtonPressed(button, _) => {
                        let input = GamepadButtonInput { gamepad, button };
                        self.inner.event(ElementState::Pressed, input);
                    }
                    EventType::ButtonReleased(button, _) => {
                        let input = GamepadButtonInput { gamepad, button };
                        self.inner.event(ElementState::Released, input);
                    }
                    EventType::ButtonChanged(button, val, _) => {
                        let (deadzone, axis) = match button {
                            GamepadButton::LeftTrigger2 => (deadzone.left, GamepadAxis::LeftZ),
                            GamepadButton::RightTrigger2 => (deadzone.right, GamepadAxis::RightZ),
                            _ => return,
                        };

                        let input = GamepadAxisInput { gamepad, axis };
                        if val.abs() <= deadzone {
                            self.axes.remove(&input);
                        } else {
                            self.axes.insert(input, val);
                        }
                    }
                    EventType::AxisChanged(axis, val, _) => {
                        let deadzone = match axis {
                            GamepadAxis::LeftStickX => deadzone.left_stick.x,
                            GamepadAxis::LeftStickY => deadzone.left_stick.y,
                            GamepadAxis::LeftZ => deadzone.left,
                            GamepadAxis::RightStickX => deadzone.right_stick.x,
                            GamepadAxis::RightStickY => deadzone.right_stick.y,
                            GamepadAxis::RightZ => deadzone.right,
                            GamepadAxis::DPadX => deadzone.d_pad.x,
                            GamepadAxis::DPadY => deadzone.d_pad.y,
                            GamepadAxis::Unknown => return,
                        };

                        let input = GamepadAxisInput { gamepad, axis };
                        if val.abs() <= deadzone {
                            self.axes.remove(&input);
                        } else {
                            self.axes.insert(input, val);
                        }
                    }
                    /* THIS MIGHT COME LATER THAN ACTUAL BUTTON EVENTS: EventType::Connected => {
                        self.gamepads.insert(gamepad);
                    } */
                    EventType::Disconnected => {
                        self.gamepads.remove(&gamepad);
                    }
                    _ => {}
                }
            }

            // auto clear event
            Event::RedrawEventsCleared if self.auto_clear => self.inner.clear(),

            _ => {}
        }
    }

    #[inline]
    pub fn set_deadzone(&mut self, gamepad: Gamepad, deadzone: DeadZone) {
        if let Some(current) = self.gamepads.get_mut(&gamepad) {
            *current = deadzone;
        }
    }

    #[inline]
    pub fn get_mut_deadzone(&mut self, gamepad: Gamepad) -> Option<&mut DeadZone> {
        self.gamepads.get_mut(&gamepad)
    }

    #[inline]
    pub fn get_deadzone(&self, gamepad: Gamepad) -> Option<&DeadZone> {
        self.gamepads.get(&gamepad)
    }

    #[inline]
    pub fn gamepads(&self) -> impl Iterator<Item = Gamepad> + '_ {
        self.gamepads.keys().copied()
    }

    #[inline]
    pub fn axis_value(&self, code: GamepadAxisInput) -> f32 {
        self.axes.get(&code).copied().unwrap_or(0.0)
    }

    pub fn axis_value_first(&self, axis: GamepadAxis) -> Option<f32> {
        self.gamepads()
            .next()
            .map(|gamepad| self.axis_value(GamepadAxisInput { gamepad, axis }))
    }
}

impl Deref for GamepadState {
    type Target = Input<GamepadButtonInput>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for GamepadState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
