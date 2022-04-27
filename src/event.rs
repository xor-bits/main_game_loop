use gilrs::GilrsBuilder;
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};
use winit::{
    error::OsError,
    event_loop::{ControlFlow, EventLoopProxy},
    window::{Window, WindowAttributes, WindowBuilder},
};

//

pub use gilrs::Event as GamePadEvent;

//

pub type Event<'a> = winit::event::Event<'a, CustomEvent>;
pub type WindowCreation = (usize, Result<Window, OsError>);

//

#[derive(Debug, Clone)]
pub enum CustomEvent {
    GamePadEvent(GamePadEvent),
    RequestWindowCreation(Box<(usize, WindowAttributes)>),
    RequestStop,
}

#[derive(Debug)]
pub struct EventLoop {
    event_loop: Option<winit::event_loop::EventLoop<CustomEvent>>,

    window_sender: Sender<WindowCreation>,
    event_sender: Sender<Event<'static>>,
}

#[derive(Debug)]
pub struct EventReceiver {
    pub window_receiver: Receiver<WindowCreation>,
    pub event_receiver: Receiver<Event<'static>>,
    pub event_proxy: EventLoopProxy<CustomEvent>,
}

//

impl EventLoop {
    pub fn new() -> (Self, EventReceiver) {
        let event_loop = winit::event_loop::EventLoop::with_user_event();
        let event_proxy = event_loop.create_proxy();
        let event_loop = Some(event_loop);

        let (window_sender, window_receiver) = channel();
        let (event_sender, event_receiver) = channel();

        let r#loop = Self {
            event_loop,

            window_sender,
            event_sender,
        };

        let receiver = EventReceiver {
            window_receiver,
            event_receiver,
            event_proxy,
        };

        (r#loop, receiver)
    }

    // Main winit event thread
    // has to be the main thread
    pub fn run(self) -> ! {
        let Self {
            mut event_loop,
            window_sender,
            event_sender,
        } = self;

        let event_loop = event_loop.take().expect("GameLoop was already running");

        Self::game_pad_loop(event_loop.create_proxy());

        event_loop.run(move |event, target, control| {
            *control = ControlFlow::Wait;

            match event {
                Event::UserEvent(CustomEvent::RequestStop) => {
                    *control = ControlFlow::Exit;
                }
                Event::UserEvent(CustomEvent::RequestWindowCreation(box (id, config))) => {
                    let mut builder = WindowBuilder::default();
                    builder.window = config;

                    if window_sender.send((id, builder.build(target))).is_err() {
                        *control = ControlFlow::Exit;
                        log::info!("Engine was stopped");
                    }
                }
                other => {
                    if let Some(other) = other.to_static() {
                        if event_sender.send(other).is_err() {
                            *control = ControlFlow::Exit;
                            log::info!("Engine was stopped");
                        }
                    }
                }
            }
        });
    }

    // GILRS gamepad event thread
    fn game_pad_loop(proxy: EventLoopProxy<CustomEvent>) {
        thread::spawn(move || {
            let mut gilrs = match GilrsBuilder::new()
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
}
