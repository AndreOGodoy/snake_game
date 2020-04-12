use opengl_graphics::GlGraphics;
use piston::input::*;

use std::collections::LinkedList;

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
    pub head_coord: Coord,
    body: LinkedList<Coord>,
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

        let head = rectangle::square(
            self.tile_size * self.head_coord.0,
            self.tile_size * self.head_coord.1,
            self.tile_size,
        );

        let body: Vec<types::Rectangle> = self.body.iter().map(|coord: &Coord| {
            graphics::rectangle::square(self.tile_size * coord.0, self.tile_size * coord.1, self.tile_size)
        }).collect();

        gl.draw(args.viewport(), |context, gl| {
            graphics::rectangle([1.0, 1.0, 1.0, 1.0], head, context.transform, gl);
            body.iter().for_each(|&body_part| graphics::rectangle([1.0, 1.0, 1.0, 1.0], body_part, context.transform, gl))
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
