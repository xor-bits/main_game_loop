use std::ops::Deref;
use winit::{
    error::OsError,
    event_loop::{
        ControlFlow, EventLoop as WinitEventLoop, EventLoopBuilder as WinitEventLoopBuilder,
        EventLoopProxy, EventLoopWindowTarget,
    },
    window::Window,
};

//

pub use gilrs::Event as GamePadEvent;

use crate::runnable::Runnable;

//

pub type Event<'a> = winit::event::Event<'a, CustomEvent>;
pub type EventLoopTarget = EventLoopWindowTarget<CustomEvent>;
pub type WindowCreation = (usize, Result<Window, OsError>);

//

#[derive(Debug, Clone)]
pub enum CustomEvent {
    GamePadEvent(GamePadEvent),
}

#[derive(Debug)]
pub struct EventLoop {
    event_loop: WinitEventLoop<CustomEvent>,
}

//

impl EventLoop {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Main winit event thread
    ///
    /// has to be called from the main thread
    #[inline]
    pub fn run<F>(self, mut event_handler: F) -> !
    where
        F: FnMut(Event, &EventLoopTarget, &mut ControlFlow) + 'static,
    {
        Self::game_pad_loop(self.event_loop.create_proxy());
        self.event_loop.run(move |e, t, c| {
            *c = ControlFlow::Poll;
            event_handler(e, t, c)
        })
    }

    /// Main winit event thread
    ///
    /// has to be called from the main thread
    #[inline]
    pub fn runnable<A>(self, mut runnable: A) -> !
    where
        A: Runnable + 'static,
    {
        self.run(move |e, t, c| {
            if let Event::RedrawEventsCleared = &e {
                runnable.draw();
            }

            runnable.event(e, t, c);
        })
    }

    // TODO:
    #[cfg(not(target_arch = "wasm32"))]
    // GILRS gamepad event thread
    fn game_pad_loop(proxy: EventLoopProxy<CustomEvent>) {
        std::thread::spawn(move || {
            let mut gilrs = match gilrs::GilrsBuilder::new()
                .add_included_mappings(true)
                .add_env_mappings(true)
                .with_default_filters(true)
                .build()
            {
                Ok(gilrs) => gilrs,
                Err(err) => {
                    log::error!("Failed to init gilrs: {err}. Gamepad input will be ignored.");
                    return;
                }
            };

            loop {
                let event = gilrs.next_event();
                // let event = InputState::deadzone(event, gilrs);

                if let Some(event) = event {
                    if let Err(..) = proxy.send_event(CustomEvent::GamePadEvent(event)) {
                        break;
                    }
                };
            }
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn game_pad_loop(_: EventLoopProxy<CustomEvent>) {}
}

impl Default for EventLoop {
    fn default() -> Self {
        Self {
            event_loop: WinitEventLoopBuilder::with_user_event().build(),
        }
    }
}

impl Deref for EventLoop {
    type Target = EventLoopTarget;

    fn deref(&self) -> &Self::Target {
        &self.event_loop
    }
}
