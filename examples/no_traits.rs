use main_game_loop::{
    event::{Event, EventLoopTarget},
    report::Reporter,
    run_app,
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

fn init(target: &EventLoopTarget) -> App {
    let _window = WindowBuilder::new().build(target).unwrap();
    let ws = WindowState::new(&_window);
    let update_loop = UpdateLoop::new(UpdateRate::PerSecond(60));

    let update_report = Reporter::new();
    let frame_report = Reporter::new();
    let event_report = Reporter::new();

    App {
        _window,
        ws,
        update_loop,

        update_report,
        frame_report,
        event_report,
    }
}

fn event(app: &mut App, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
    app.event_report.time(|| {
        app.ws.event(&event);
    });

    if app.ws.should_close {
        *control = ControlFlow::Exit;
    }
}

fn draw(app: &mut App) {
    app.update_loop.update(|| {
        app.update_report.time(|| {
            // update();
        });
    });

    app.frame_report.time(|| {
        std::thread::sleep(Duration::from_millis(3));
        // draw();
    });

    if app.frame_report.should_report() {
        let reporters = [
            ("UPDATE", &mut app.update_report),
            ("FRAME", &mut app.frame_report),
            ("EVENT", &mut app.event_report),
        ];
        log::debug!("\n{}", Reporter::report_all("5.0s", reporters));
    }
}

fn main() {
    env_logger::init();
    run_app!(init, event, draw);
}
