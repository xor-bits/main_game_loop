use std::future::Future;

//

pub mod event;
pub mod report;
pub mod state;
pub mod update;

//

pub fn as_async<F>(f: F)
where
    F: Future<Output = ()> + 'static,
{
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(f);
    #[cfg(not(target_arch = "wasm32"))]
    pollster::block_on(f);
}

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
