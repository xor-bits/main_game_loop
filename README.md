<div align="center">

# main_game_loop

[![dependency status](https://deps.rs/repo/github/Overpeek/main_game_loop/status.svg)](https://deps.rs/repo/github/Overpeek/main_game_loop)

<!-- [![build status](https://github.com/Overpeek/main_game_loop/actions/workflows/rust.yml/badge.svg)](https://github.com/Overpeek/main_game_loop/actions) -->

</div>

### Example usage with some random `Engine`

```rust
struct App {
    window: Window,
    ws: WindowState,
    update_loop: UpdateLoop,
}

impl App {
    fn init(target: &EventLoopTarget) -> Self {
        let window = WindowBuilder::new().build(target).unwrap();
        let ws = WindowState::new(&window);
        let update_loop = UpdateLoop::new(UpdateRate::PerSecond(60));

        Self {
            window,
            ws,
            update_loop,
        }
    }
}

impl Runnable for App {
    fn event(&mut self, event: Event, _: &EventLoopTarget, control: &mut ControlFlow) {
        self.ws.event(&event);

        if self.ws.should_close {
            *control = ControlFlow::Exit;
        }
    }

    fn draw(&mut self) {
        self.update_loop.update(|| {
            // update();
        });

        // draw();
    }
}

main_app!(App);
```
