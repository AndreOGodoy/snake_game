use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use piston::Button;

mod snake;
use snake::Snake;

mod food;
use food::Food;

use crate::coord::Coord;

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
    food: Food,
}

impl Game {
    pub fn new(gl: GlGraphics, tile_size: f64) -> Self {
        Game {
            gl,
            snake: Snake::new(Coord(10.0, 10.0), tile_size),
            food: Food::new(Coord(10.0, 10.0), tile_size),
        }
    }

    fn render_background(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_context, gl| {
            graphics::clear([90.0 / 255.0, 177.0 / 255.0, 187.0 / 255.0, 1.0], gl);
        });
    }

    fn head_in_food(&self) -> bool {
        if self.snake.head_coord == self.food.coord {
            true
        } else {
            false
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        self.render_background(&args);
        self.food.render(&args, &mut self.gl);
        self.snake.render(&args, &mut self.gl);
    }

    pub fn update(&mut self) {
        self.snake.update();
        if self.head_in_food() {
            self.snake.eat();
            self.food.update();
        }
    }

    pub fn register(&mut self, button: &Button) {
        self.snake.register(&button);
    }
}
