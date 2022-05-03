use std::future::Future;

//

pub mod event;
pub mod report;
pub mod runnable;
pub mod state;
pub mod update;

//

#[inline(always)]
pub fn as_async<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(f);
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(f);
}

#[inline(always)]
pub fn init_log() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init_with_level(log::Level::Debug);
    }
}

//

#[macro_export]
macro_rules! main_app {
    ($app:tt) => {
        fn main() {
            main_game_loop::init_log();
            let target = main_game_loop::event::EventLoop::new();
            let app = $app::init(&target);
            target.runnable(app);
        }
    };

    (async $app:tt) => {
        async fn __main_run() {
            let target = main_game_loop::event::EventLoop::new();
            let app = $app::init(&target).await;
            target.runnable(app);
        }

        fn main() {
            main_game_loop::init_log();
            main_game_loop::as_async(__main_run());
        }
    };
}
