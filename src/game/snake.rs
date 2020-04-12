use opengl_graphics::GlGraphics;
use piston::input::*;

use crate::coord::Coord;

#[derive(PartialEq, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    HOLD,
}

pub struct Snake {
    coord: Coord,
    tile_size: f64,
    direction: Direction,
}

impl Snake {
    pub fn new(coord: Coord, tile_size: f64) -> Self {
        Snake {
            coord,
            tile_size,
            direction: Direction::HOLD,
        }
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        let head = graphics::rectangle::square(
            self.tile_size * self.coord.0,
            self.tile_size * self.coord.1,
            self.tile_size,
        );

        gl.draw(args.viewport(), |context, gl| {
            graphics::rectangle([1.0, 1.0, 1.0, 1.0], head, context.transform, gl)
        });
    }

    pub fn update(&mut self) {
        match self.direction {
            Direction::UP => self.coord = self.coord - Coord(0.0, 1.0),
            Direction::DOWN => self.coord = self.coord + Coord(0.0, 1.0),
            Direction::RIGHT => self.coord = self.coord + Coord(1.0, 0.0),
            Direction::LEFT => self.coord = self.coord - Coord(1.0, 0.0),
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
            _ => last_direction,
        };
    }
}
