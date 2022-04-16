use report::Reporter;
use std::{
    time::{Duration, Instant},  ops::{Deref, DerefMut},
};
use winit::{
    dpi::PhysicalPosition,
    event::{ WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};
use gilrs::{ Gilrs, GilrsBuilder};

//

pub use gilrs::Event as GilrsEvent;
pub use winit::event::Event as WinitEvent;

//

pub mod report;
pub mod io;

//

pub trait Runnable<E: AnyEngine + 'static> {
    #[allow(unused_variables)]
    fn update(&mut self, gl: &mut GameLoop<E>) {}

    #[allow(unused_variables)]
    fn event(&mut self, gl: &mut GameLoop<E>, event: &Event) {
        if let Event::WinitEvent(WinitEvent::WindowEvent { event: WindowEvent::CloseRequested, ..}) = event {
            gl.stop();
        }
    }

    #[allow(unused_variables)]
    fn draw(&mut self, gl: &mut GameLoop<E>, frame: &mut E::Frame, delta: f32) {}
}

pub trait AnyEngine: Sized {
    type Frame;
    fn get_frame(&mut self) -> Self::Frame;
    fn finish_frame(&mut self, frame: Self::Frame);

    fn get_window(&self) -> &Window;

    fn build_game_loop(self) -> GameLoop<Self> {
        GameLoop::new(self)
    }

    fn take_event_loop(&mut self) -> EventLoop<()>;
}

//

#[derive(Debug)]
pub struct GameLoop<E: AnyEngine + 'static> {
    pub engine: E,

    pub stop: bool,

    //
    pub frame_reporter: Reporter,
    pub update_reporter: Reporter,

    gilrs: Option<Gilrs>,

    // window size
    pub size: (f32, f32),

    // window aspect ratio
    pub aspect: f32,

    // is cursor inside the window?
    pub cursor_in: bool,

    // cursor position
    pub cursor_pos: PhysicalPosition<f64>,

    // window scaling factor
    pub scale_factor: f64,

    // update interval
    pub interval: Duration,
}

#[derive(Debug, PartialEq)]
pub enum Event<'e> {
    /// Controller/gamepad/joystick related events
    GilrsEvent(GilrsEvent),

    /// Window/Keyboard/Cursor/Device events
    WinitEvent(WinitEvent<'e, ()>)
}

//

impl<E: AnyEngine + 'static> GameLoop<E> {
    pub fn new(engine: E) -> Self {
        let frame_reporter = Reporter::new_with_interval(Duration::from_secs_f32(5.0));
        let update_reporter = Reporter::new_with_interval(Duration::from_secs_f32(5.0));

        let gilrs = match GilrsBuilder::new().add_included_mappings(true).add_env_mappings(true).with_default_filters(true).build() {
            Ok(gilrs) => Some(gilrs),
            Err(err) => {
                log::error!("Failed to init gilrs: {err}. Gamepad input will be ignored.");
                None
            },
        };

        let window = engine.get_window();
        let scale_factor = window.scale_factor();
        let size = window.inner_size().to_logical(scale_factor);

        let size = (size.width, size.height);
        let aspect = size.0 / size.1;
        let interval = Duration::from_secs_f64(1.0 / 60.0);
        let stop = false;
        let cursor_in = false;
        let cursor_pos = Default::default();

        Self {
            engine,

            stop,

            frame_reporter,
            update_reporter,

            gilrs,

            size,
            aspect,
            cursor_in,
            cursor_pos,
            scale_factor,
            interval,
        }
    }

    pub fn stop(&mut self) {
        self.stop = true
    }

    pub fn run(mut self, mut app: impl Runnable<E> + 'static) -> ! {
        let mut previous = Instant::now();
        let mut lag = Duration::from_secs_f64(0.0);

        self.engine.get_window().set_visible(true);
        let event_loop = self.engine.take_event_loop();

        event_loop.run(move |event, _, control| {
                *control = if self.stop {
                    ControlFlow::Exit
                } else {
                    ControlFlow::Poll
                };

                if let Some(gilrs) = self.gilrs.as_mut() {
                    let event = gilrs.next_event();
                    // let event = InputState::deadzone(event, gilrs);
                    if let Some(event) = event {
                        app.event(&mut self, &Event::GilrsEvent(event));
                    };
                }

                match &event {
                    WinitEvent::WindowEvent {
                        event: WindowEvent::CursorEntered { .. },
                        ..
                    } => self.cursor_in = true,
                    WinitEvent::WindowEvent {
                        event: WindowEvent::CursorLeft { .. },
                        ..
                    } => self.cursor_in = false,
                    WinitEvent::WindowEvent {
                        event: WindowEvent::CursorMoved { position, .. },
                        ..
                    } => {
                        self.cursor_pos = *position;
                    }
                    WinitEvent::WindowEvent {
                        event: WindowEvent::Resized(s),
                        ..
                    } => {
                        self.size = (s.width as f32, s.height as f32);
                        let s = s.to_logical::<f32>(self.scale_factor);
                        self.aspect = s.width / s.height;
                    }
                    WinitEvent::RedrawRequested(_) => {
                        // main game loop source:
                        //  - https://gameprogrammingpatterns.com/game-loop.html
                        let elapsed = previous.elapsed();
                        previous = Instant::now();
                        lag += elapsed;

                        // updates
                        while lag >= self.interval {
                            let timer = self.update_reporter.begin();
                            app.update(&mut self);
                            self.update_reporter.end(timer);
                            lag -= self.interval;
                        }

                        // frames
                        let timer = self.frame_reporter.begin();
                        {
                            
                            let mut frame = self.engine.get_frame();
                            let delta = lag.as_secs_f32() / self.interval.as_secs_f32();
                            app.draw(
                                &mut self,
                                &mut frame,
                                delta,
                            );
                            self.engine.finish_frame(frame);
                        }
                        let should_report = self.frame_reporter.end(timer);

                        // reports
                        if should_report {
                            let int = self.frame_reporter.report_interval();
                            let (u_int, u_per_sec) = self.update_reporter.last_string();
                            let (f_int, f_per_sec) = self.frame_reporter.last_string();

                            #[cfg(debug_assertions)]
                            const DEBUG: &str = "debug build";
                            #[cfg(not(debug_assertions))]
                            const DEBUG: &str = "release build";

                            log::debug!(
                                "Report ({:?})({})\n        per second @ time per\nUPDATES: {:>9} @ {}\nFRAMES: {:>10} @ {}",
                                int,
                                DEBUG,
                                u_per_sec,
                                u_int,
                                f_per_sec,
                                f_int
                            );
                        }

                        return;
                    }
                    WinitEvent::MainEventsCleared => {
                        self.engine.get_window().request_redraw();
                    }
                    _ => {}
                }

                app.event(&mut self, &Event::WinitEvent(event));
            })
    }
}

//

impl<E: AnyEngine> Deref for GameLoop<E> {
    type Target = E;

    fn deref(&self) -> &Self::Target {
        &self.engine
    }
}

impl<E: AnyEngine> DerefMut for GameLoop<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.engine
    }
}

#[cfg(feature = "gl")]
impl<E: AnyEngine + glium::backend::Facade> glium::backend::Facade for GameLoop<E> {
    fn get_context(&self) -> &std::rc::Rc<glium::backend::Context> {
        self.engine.get_context()
    }
}