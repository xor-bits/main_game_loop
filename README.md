<div align="center">

# main_game_loop

[![dependency status](https://deps.rs/repo/github/Overpeek/main_game_loop/status.svg)](https://deps.rs/repo/github/Overpeek/main_game_loop)

<!-- [![build status](https://github.com/Overpeek/main_game_loop/actions/workflows/rust.yml/badge.svg)](https://github.com/Overpeek/main_game_loop/actions) -->

</div>

### Example usage with some random `Engine`

```rust
fn run(mut engine: Engine) {
    let window = engine.create_window(WindowBuilder::new())unwrap();
    let mut update_loop = UpdateLoop::new(UpdateRate::PerSecond(60));

    loop {
        if let Some(event) = engine.poll() {
            event(event);
        } else {
            let delta = update_loop.update(|| {
                update();
            });

            draw();

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
```
