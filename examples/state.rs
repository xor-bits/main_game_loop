use instant::Duration;
use main_game_loop::{
    event::{Event, EventLoopTarget},
    prelude::{FPCam, KeyboardState, WindowState},
    run_app,
    runnable::Runnable,
    state::gamepad::GamepadState,
};
use std::thread::sleep;
use winit::{event_loop::ControlFlow, window::Window};

//

struct App {
    _window: Window,
    window: WindowState,
    keyboard: KeyboardState,
    gamepad: GamepadState,
    fpcam: FPCam,
}

//

impl App {
    fn init(target: &EventLoopTarget) -> Self {
        let _window = Window::new(target).unwrap();
        let window = WindowState::new(&_window);
        let keyboard = KeyboardState::new();
        let gamepad = GamepadState::new();
        let fpcam = FPCam::new();

        Self {
            _window,
            window,
            keyboard,
            gamepad,
            fpcam,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.window.event(&event);
        self.keyboard.event(&event);
        self.gamepad.event(&event);
        if self.window.focused {
            self.fpcam.event(&event);
        }

        if self.window.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        println!(
            "Window state: {:?}\nKeyboard state: {:#?}\nGamepad state: {:?}\nFPCam state: {:?}\n\n",
            self.window, self.keyboard, self.gamepad, self.fpcam
        );
        self.keyboard.clear();
        self.gamepad.clear();
        sleep(Duration::from_millis(10));
    }
}

fn main() {
    run_app!(App);
}
