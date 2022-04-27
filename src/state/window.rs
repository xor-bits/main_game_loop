use std::time::Duration;
use winit::dpi::PhysicalPosition;

//

// TODO:
#[derive(Debug, Clone, Copy)]
pub struct WindowState {
    /// window size
    pub size: (f32, f32),

    /// window aspect ratio
    pub aspect: f32,

    /// is cursor inside the window?
    pub cursor_in: bool,

    /// cursor position
    pub cursor_pos: PhysicalPosition<f64>,

    /// window scaling factor
    pub scale_factor: f64,

    /// update interval
    pub interval: Duration,
}
