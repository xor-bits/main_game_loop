#![forbid(unsafe_code)]

//

pub use gilrs;
pub use glam;
pub use instant;
pub use log;
pub use rustc_hash;
pub use winit;

//

pub mod event;
pub mod prelude;
pub mod report;
pub mod runnable;
pub mod state;
pub mod update;

//

pub fn should_draw(event: &prelude::Event) -> bool {
    matches!(event, prelude::Event::RedrawEventsCleared)
}

#[macro_export]
macro_rules! run_app {
    (async $init:expr, $event:expr, $draw:expr) => {
        run_app! { => { $init }, { .await }, { $event }, { $draw } }
    };

    ($init:expr, $event:expr, $draw:expr) => {
        run_app! { => { $init }, {}, { $event }, { $draw } }
    };

    ($app:ident) => {
        run_app! { => { $app::init }, {}, { $app::event }, { $app::draw } }
    };

    (=> { $($init:tt)* }, { $($init_op:tt)* }, { $($event:tt)* }, { $($draw:expr)* }) => {
        let target = main_game_loop::event::EventLoop::new();
        let mut app = ($($init)*)(&target)$($init_op)*;

        target.run(move |e, t, c| {
            if main_game_loop::should_draw(&e) {
                ($($draw)*)(&mut app);
            }

            ($($event)*)(&mut app, e, t, c);
        })
    };
}
