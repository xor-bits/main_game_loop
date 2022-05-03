use crate::event::{Event, EventLoopTarget};
use winit::event_loop::ControlFlow;

//

pub trait Runnable {
    fn event(&mut self, event: Event, target: &EventLoopTarget, control: &mut ControlFlow);

    fn draw(&mut self);
}
