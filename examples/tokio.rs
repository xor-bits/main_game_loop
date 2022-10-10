use main_game_loop::{
    event::{Event, EventLoopTarget},
    prelude::{EventLoop, WindowState},
    should_draw,
};
use tokio::runtime::Builder;
use winit::{event_loop::ControlFlow, window::Window};

//

struct App {
    window: Window,
    ws: WindowState,
}

async fn init(target: &EventLoopTarget) -> App {
    let window = Window::new(target).unwrap();
    let ws = WindowState::new(&window);

    App { window, ws }
}

async fn event(app: &mut App, event: Event<'_>, _: &EventLoopTarget, control: &mut ControlFlow) {
    let _ = &app.window;
    app.ws.event(&event);

    if app.ws.should_close {
        *control = ControlFlow::Exit;
    }
}

async fn draw(_: &mut App) {}

fn main() {
    env_logger::init();

    let runtime = Builder::new_current_thread().enable_all().build().unwrap();
    let target = EventLoop::new();
    let mut app = runtime.block_on(init(&target));

    target.run(move |e, t, c| {
        runtime.block_on(async {
            if should_draw(&e) {
                draw(&mut app).await;
            }
            event(&mut app, e, t, c).await;
        })
    });
}
