use main_game_loop::{
    engine::AnyEngine,
    event::{EventLoop, EventReceiver},
    report::Reporter,
    update::{UpdateLoop, UpdateRate},
};
use std::{thread, time::Duration};
use winit::window::WindowBuilder;

//

fn run(mut engine: Engine) {
    let _window = engine.create_window(WindowBuilder::new()).unwrap();
    let mut update_loop = UpdateLoop::new(UpdateRate::PerSecond(60));

    let mut update_report = Reporter::new();
    let mut frame_report = Reporter::new();
    let mut event_report = Reporter::new();

    loop {
        if engine.poll().is_some() {
            let timer = event_report.begin();
            // event();
            event_report.end(timer);
        } else {
            update_loop.update(|| {
                let timer = update_report.begin();
                // update();
                update_report.end(timer);
            });

            let timer = frame_report.begin();
            std::thread::sleep(Duration::from_millis(3));
            // draw();
            let should_report = frame_report.end(timer);

            if should_report {
                log::debug!(
                    "\n{}",
                    Reporter::report_all(
                        "5.0s",
                        &[
                            ("UPDATE", &update_report),
                            ("FRAME", &frame_report),
                            ("EVENT", &event_report),
                        ],
                    )
                );
            }
        }
    }
}

fn main() {
    env_logger::init();
    Engine::new().run(run);
}

// --------------
// No Engine impl
// --------------

struct Engine {
    event_receiver: Option<EventReceiver>,
}

//

impl Engine {
    pub fn new() -> Self {
        Self {
            event_receiver: None,
        }
    }
}

impl AnyEngine for Engine {
    type Frame = ();
    fn get_frame(&mut self) -> Self::Frame {}
    fn finish_frame(&mut self, _: Self::Frame) {}

    fn run<F>(mut self, f: F) -> !
    where
        F: FnOnce(Self) + Send + 'static,
    {
        let (game_loop, event_receiver) = EventLoop::new();
        self.event_receiver = Some(event_receiver);
        thread::spawn(move || f(self));
        game_loop.run()
    }

    fn event_receiver(&mut self) -> &mut EventReceiver {
        self.event_receiver.as_mut().expect("GameLoop not running")
    }
}
