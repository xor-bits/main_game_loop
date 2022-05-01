use main_game_loop::{
    event::{Event, EventLoop, EventLoopTarget},
    report::Reporter,
    state::window::WindowState,
    update::{UpdateLoop, UpdateRate},
};
use std::time::Duration;
use winit::{
    event_loop::ControlFlow,
    window::{Window, WindowBuilder},
};

//

struct App {
    window: Window,
    ws: WindowState,
    update_loop: UpdateLoop,

    update_report: Reporter,
    frame_report: Reporter,
    event_report: Reporter,
}

impl App {
    fn init(target: &EventLoopTarget) -> Self {
        let window = WindowBuilder::new().build(target).unwrap();
        let ws = WindowState::new(&window);
        let update_loop = UpdateLoop::new(UpdateRate::PerSecond(60));

        let update_report = Reporter::new();
        let frame_report = Reporter::new();
        let event_report = Reporter::new();

        Self {
            window,
            ws,
            update_loop,

            update_report,
            frame_report,
            event_report,
        }
    }

    fn event(&mut self, event: Event, control: &mut ControlFlow) {
        self.event_report.time(|| {
            *control = ControlFlow::Poll;
            let _ = &self.window;
            self.ws.event(&event);
        });

        if self.ws.should_close {
            *control = ControlFlow::Exit;
            return;
        }

        if let Event::RedrawEventsCleared = event {
            self.update_loop.update(|| {
                self.update_report.time(|| {
                    // update();
                });
            });

            self.frame_report.time(|| {
                std::thread::sleep(Duration::from_millis(3));
                // draw();
            });

            if self.frame_report.should_report() {
                let reporters = [
                    ("UPDATE", &mut self.update_report),
                    ("FRAME", &mut self.frame_report),
                    ("EVENT", &mut self.event_report),
                ];
                log::debug!("\n{}", Reporter::report_all("5.0s", reporters));
            }
        }
    }
}

fn main() {
    env_logger::init();
    let target = EventLoop::new();
    let mut app = App::init(&target);
    target.run(move |event, _, control| {
        app.event(event, control);
    });
}
