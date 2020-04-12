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
    pub snake: Snake,
    food: Food,
    screen_size: (f64, f64),
    tile_size: f64,
}

impl Game {
    pub fn new(gl: GlGraphics, tile_size: f64, screen_size: (f64, f64)) -> Self {
        let (max_x, max_y) = screen_size;
        Game {
            gl,
            snake: Snake::new(Coord(10.0, 10.0), tile_size),
            food: Food::new(Coord(max_x / tile_size, max_y / tile_size), tile_size),
            screen_size,
            tile_size,
        }
    }

    fn render_background(&mut self, args: &RenderArgs) {
        let background_color = [231.0 / 255.0, 211.0 / 255.0, 159.0 / 255.0, 1.0];

        self.gl.draw(args.viewport(), |_context, gl| {
            graphics::clear(background_color, gl);
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
        self.food.set_max_coord(Coord(
            self.screen_size.0 / self.tile_size,
            self.screen_size.1 / self.tile_size,
        ));
        if self.head_in_food() {
            self.snake.eat();
            self.food.update();
            if self.snake.is_in_body(self.food.coord) {
                self.food.update();
            }
        }
        self.snake.update();
    }

    pub fn register(&mut self, button: &Button) {
        self.snake.register(&button);
    }

    pub fn set_max_size(&mut self, screen_size: (f64, f64)) {
        self.screen_size = screen_size;
    }
}
