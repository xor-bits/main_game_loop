<div align="center">

# game_loop

[![dependency status](https://deps.rs/repo/github/Overpeek/game_loop/status.svg)](https://deps.rs/repo/github/Overpeek/game_loop)

<!-- [![build status](https://github.com/Overpeek/game_loop/actions/workflows/rust.yml/badge.svg)](https://github.com/Overpeek/game_loop/actions) -->

</div>

### Example usage with some random `Engine`

```rust
fn main() {
	let engine = Engine::new();
	engine.build_game_loop().run::<App>();
}

struct App {
	vbo: VertexBuffer,
	shader: Shader,
}

impl Runnable<Engine> for App {
    fn init(_: &mut GameLoop<Engine>) -> Self {
        Self {
            vbo: VertexBuffer::new(),
			shader: Shader::new(),
        }
    }

    fn draw(&mut self, gl: &mut GameLoop<Engine>, frame: &mut Frame, delta: f32) {
        frame
            .main_render_pass()
            .bind_vbo(&self.vbo)
            .bind_shader(&self.shader)
            .draw(0..6, 0..1);
    }
}
```
