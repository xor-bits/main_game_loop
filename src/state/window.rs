use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::WindowEvent,
    window::{Window, WindowId},
};

use crate::event::Event;

//

// TODO:
#[derive(Debug, Clone, Copy)]
pub struct WindowState {
    /// window size
    pub size: PhysicalSize<u32>,

    /// window aspect ratio
    pub aspect: f32,

    /// is the window focused
    pub focused: bool,

    /// is cursor inside the window?
    pub cursor_in: bool,

    /// cursor position
    pub cursor_pos: PhysicalPosition<f64>,

    /// window scaling factor
    pub scale_factor: f64,

    /// identifier for the window
    /// this struct was initialized for
    pub id: WindowId,
}

//

impl WindowState {
    pub fn new(window: &Window) -> Self {
        let size = window.inner_size();

        Self {
            size,
            aspect: Self::aspect(size),
            focused: Default::default(),
            cursor_in: Default::default(),
            cursor_pos: Default::default(),
            scale_factor: window.scale_factor(),
            id: window.id(),
        }
    }

    pub fn event(&mut self, event: &Event) {
        if !self.filter(event) {
            return;
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CursorEntered { .. },
                ..
            } => {
                self.cursor_in = true;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorLeft { .. },
                ..
            } => {
                self.cursor_in = false;
            }
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position, .. },
                ..
            } => {
                self.cursor_pos = *position;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                self.size = *size;
                self.aspect = Self::aspect(*size);
            }
            Event::WindowEvent {
                event: WindowEvent::Focused(focused),
                ..
            } => {
                self.focused = *focused;
            }
            _ => {}
        }
    }

    fn aspect(size: PhysicalSize<u32>) -> f32 {
        if size.height == 0 {
            0.0
        } else {
            size.width as f32 / size.height as f32
        }
    }

    fn filter(&self, event: &Event) -> bool {
        if let Event::WindowEvent { window_id, .. } = event {
            *window_id == self.id
        } else {
            false
        }
    }
}
