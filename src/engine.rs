use crate::{event::EventReceiver, CustomEvent, Event};
use std::{
    sync::mpsc::{RecvTimeoutError, TryRecvError},
    time::{Duration, Instant},
};
use winit::{
    error::OsError,
    window::{Window, WindowBuilder},
};

//

pub trait AnyEngine: Sized {
    type Frame;

    /// Get a new frame from the game engine
    ///
    /// This is where the game engine user
    /// should draw to
    ///
    /// No two frames should exist at the
    /// same time
    fn get_frame(&mut self) -> Self::Frame;

    /// Finish a frame
    ///
    /// Game engine can for example: submit
    /// the commands it got or something
    fn finish_frame(&mut self, frame: Self::Frame);

    fn run<F>(self, f: F) -> !
    where
        F: FnOnce(Self) + Send + 'static;

    fn event_receiver(&mut self) -> &mut EventReceiver;

    fn wait(&mut self) -> Event {
        match self.event_receiver().event_receiver.recv() {
            Ok(event) => event,
            Err(_) => unreachable!(),
        }
    }

    fn wait_deadline(&mut self, deadline: Instant) -> Option<Event> {
        match self.event_receiver().event_receiver.recv_deadline(deadline) {
            Ok(event) => Some(event),
            Err(RecvTimeoutError::Disconnected) => unreachable!(),
            Err(RecvTimeoutError::Timeout) => None,
        }
    }

    fn wait_timeout(&mut self, timeout: Duration) -> Option<Event> {
        match self.event_receiver().event_receiver.recv_timeout(timeout) {
            Ok(event) => Some(event),
            Err(RecvTimeoutError::Disconnected) => unreachable!(),
            Err(RecvTimeoutError::Timeout) => None,
        }
    }

    fn poll(&mut self) -> Option<Event> {
        match self.event_receiver().event_receiver.try_recv() {
            Ok(event) => Some(event),
            Err(TryRecvError::Disconnected) => unreachable!(),
            Err(TryRecvError::Empty) => None,
        }
    }

    fn create_window(&mut self, builder: WindowBuilder) -> Result<Window, OsError> {
        self.event_receiver()
            .event_proxy
            .send_event(CustomEvent::RequestWindowCreation(Box::new((
                0,
                builder.window,
            ))))
            .unwrap();
        self.event_receiver().window_receiver.recv().unwrap().1
    }
}
