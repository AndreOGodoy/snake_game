use opengl_graphics::GlGraphics;
use piston::input::*;

use std::collections::LinkedList;

use crate::coord::Coord;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    HOLD,
}

pub struct Snake {
    pub head_coord: Coord,
    pub body: LinkedList<Coord>,
    tile_size: f64,
    direction: Direction,
}

impl Snake {
    pub fn new(head_coord: Coord, tile_size: f64) -> Self {
        Snake {
            head_coord,
            body: LinkedList::default(),
            tile_size,
            direction: Direction::HOLD,
        }
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        let snake_color = [215.0 / 255.0, 56.0 / 255.0, 94.0 / 255.0, 1.0];

        let head = rectangle::square(
            self.tile_size * self.head_coord.0,
            self.tile_size * self.head_coord.1,
            self.tile_size,
        );

        let body: Vec<types::Rectangle> = self
            .body
            .iter()
            .map(|coord: &Coord| {
                graphics::rectangle::square(
                    self.tile_size * coord.0,
                    self.tile_size * coord.1,
                    self.tile_size,
                )
            })
            .collect();

        gl.draw(args.viewport(), |context, gl| {
            graphics::rectangle(snake_color, head, context.transform, gl);
            body.iter().for_each(|&body_part| {
                graphics::rectangle(snake_color, body_part, context.transform, gl)
            })
        });
    }

    pub fn update(&mut self) {
        let mut new_head: Coord = self.head_coord;

        match self.direction {
            Direction::UP => new_head = self.head_coord - Coord(0.0, 1.0),
            Direction::DOWN => new_head = self.head_coord + Coord(0.0, 1.0),
            Direction::RIGHT => new_head = self.head_coord + Coord(1.0, 0.0),
            Direction::LEFT => new_head = self.head_coord - Coord(1.0, 0.0),
            Direction::HOLD => (),
        }

        self.body.push_front(self.head_coord);
        self.body.pop_back();
        self.head_coord = new_head;
    }

    pub fn eat(&mut self) {
        let head = self.head_coord;

        if self.body.is_empty() {
            self.body.push_back(head);
        } else {
            self.body.push_back(*self.body.back().unwrap());
        }
    }

    pub fn register(&mut self, button: &Button) {
        let last_direction = self.direction;

        self.direction = match button {
            Button::Keyboard(Key::Up) if last_direction != Direction::DOWN => Direction::UP,
            Button::Keyboard(Key::Down) if last_direction != Direction::UP => Direction::DOWN,
            Button::Keyboard(Key::Right) if last_direction != Direction::LEFT => Direction::RIGHT,
            Button::Keyboard(Key::Left) if last_direction != Direction::RIGHT => Direction::LEFT,
            _ => last_direction,
        };
    }

    pub fn is_in_body(&self, coord: Coord) -> bool {
        self.body.iter().any(|body_part| *body_part == coord)
    }
}
