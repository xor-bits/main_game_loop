use main_game_loop::{
    event::{Event, EventLoopTarget},
    report::Reporter,
    run_app,
    runnable::Runnable,
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
    _window: Window,
    ws: WindowState,
    update_loop: UpdateLoop,

    update_report: Reporter,
    frame_report: Reporter,
    event_report: Reporter,
}

impl App {
    fn init(target: &EventLoopTarget) -> Self {
        let _window = WindowBuilder::new().build(target).unwrap();
        let ws = WindowState::new(&_window);
        let update_loop = UpdateLoop::new(UpdateRate::PerSecond(60));

        let update_report = Reporter::new();
        let frame_report = Reporter::new();
        let event_report = Reporter::new();

        Self {
            _window,
            ws,
            update_loop,

            update_report,
            frame_report,
            event_report,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.event_report.time(|| {
            self.ws.event(&event);
        });

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
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

fn main() {
    env_logger::init();
    run_app!(App);
}
