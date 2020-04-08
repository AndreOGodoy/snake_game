use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use piston::Button;

mod snake;
use snake::Snake;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    pub fn new(tile_size: u32, gl: GlGraphics) -> Self {
        let converted_tile = tile_size as f64;

        Game {
            gl,
            snake: Snake::new(10.0, 10.0, converted_tile),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_context, gl| {
            graphics::clear([90.0 / 255.0, 177.0 / 255.0, 187.0 / 255.0, 1.0], gl);
        });

        self.snake.render(&args, &mut self.gl);
    }

    pub fn update(&mut self) {
        self.snake.update();
    }

    pub fn register(&mut self, button: &Button) {
        self.snake.register(&button);
    }
}
