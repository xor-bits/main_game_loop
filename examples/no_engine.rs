use main_game_loop::{
    engine::AnyEngine,
    event::{EventLoop, EventReceiver},
    report::Reporter,
    state::window::WindowState,
    update::{UpdateLoop, UpdateRate},
};
use std::{thread, time::Duration};
use winit::window::WindowBuilder;

//

fn run(mut engine: Engine) {
    let window = engine.create_window(WindowBuilder::new()).unwrap();
    let mut ws = WindowState::new(&window);
    let mut update_loop = UpdateLoop::new(UpdateRate::PerSecond(60));

    let mut update_report = Reporter::new();
    let mut frame_report = Reporter::new();
    let mut event_report = Reporter::new();

    loop {
        while let Some(event) = engine.poll() {
            event_report.time(|| {
                ws.event(&event);
            });

            if ws.should_close {
                return;
            }
        }

        update_loop.update(|| {
            update_report.time(|| {
                // update();
            });
        });

        frame_report.time(|| {
            std::thread::sleep(Duration::from_millis(3));
            // draw();
        });

        if frame_report.should_report() {
            let reporters = [
                ("UPDATE", &update_report),
                ("FRAME", &frame_report),
                ("EVENT", &event_report),
            ];
            log::debug!("\n{}", Reporter::report_all("5.0s", &reporters,));
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
