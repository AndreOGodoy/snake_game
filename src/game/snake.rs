use opengl_graphics::GlGraphics;
use piston::input::*;

#[derive(PartialEq, Clone)]
enum Direction { UP, DOWN, LEFT, RIGHT, HOLD}

pub struct Snake {
    x: f64,
    y: f64,
    tile_size: f64,
    direction: Direction
}

impl Snake {
    pub fn new(x: f64, y: f64, tile_size: f64) -> Self {
        Snake {x, y, tile_size, direction: Direction::HOLD}
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        let head = graphics::rectangle::square(self.tile_size * self.x, self.tile_size * self.y, self.tile_size);

        gl.draw(args.viewport(), |context, gl| {
            graphics::rectangle([1.0, 1.0, 1.0, 1.0], head, context.transform, gl)
        });
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::UP => (self.y = self.y - 1.0),
            Direction::DOWN => (self.y = self.y + 1.0),
            Direction::RIGHT => (self.x = self.x + 1.0),
            Direction::LEFT => (self.x = self.x - 1.0),
            Direction::HOLD => (),
        }
    }

    pub fn register(&mut self, button: &Button) {
        let last_direction = self.direction.clone();

        self.direction = match button {
            Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            _ => last_direction
        };
    }

}
