use crate::event::{Event, EventLoop, Target};

//

pub trait AnyEngine: Sized {
    fn run<T, FInit, FEvent>(self, init: FInit, event: FEvent) -> !
    where
        Self: 'static,
        T: 'static,
        FInit: FnOnce(&mut Self, &Target) -> T + 'static,
        FEvent: FnMut(&mut T, &mut Self, Event, &Target) + 'static,
    {
        EventLoop::new().run(self, init, event)
    }
}
