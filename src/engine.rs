use crate::{event::EventReceiver, CustomEvent, Event};
use std::{
    sync::mpsc::{RecvTimeoutError, TryRecvError},
    time::Duration,
};
use winit::{
    error::OsError,
    window::{Window, WindowBuilder},
};

//

pub trait AnyEngine: Sized {
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

    /* TODO: feature deadline_api: fn wait_deadline(&mut self, deadline: Instant) -> Option<Event> {
        match self.event_receiver().event_receiver.recv_deadline(deadline) {
            Ok(event) => Some(event),
            Err(RecvTimeoutError::Disconnected) => unreachable!(),
            Err(RecvTimeoutError::Timeout) => None,
        }
    } */

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
