extern crate rand;

use crate::coord::Coord;
use crate::graphics;
use crate::opengl_graphics::GlGraphics;
use crate::piston::input::RenderArgs;

use rand::Rng;

pub struct Food {
    pub coord: Coord,
    tile_size: f64,
    max_coord: Coord,
}

impl Food {
    pub fn new(max_coord: Coord, tile_size: f64) -> Self {
        let mut rng = rand::thread_rng();
        let (x, y) = (
            rng.gen_range(0, max_coord.1 as i32 - 1),
            rng.gen_range(0, max_coord.0 as i32 - 1),
        );

        Food {
            coord: Coord(x as f64, y as f64),
            tile_size,
            max_coord,
        }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        self.coord = Coord(
            rng.gen_range(0, self.max_coord.1 as i32 - 1) as f64,
            rng.gen_range(0, self.max_coord.0 as i32 - 1) as f64,
        );
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        let food_color = [82.0 / 255.0, 45.0 / 255.0, 91.0 / 255.0, 1.0];

        let food_square = graphics::rectangle::square(
            self.coord.0 * self.tile_size,
            self.coord.1 * self.tile_size,
            self.tile_size,
        );
        gl.draw(args.viewport(), |context, gl| {
            graphics::rectangle(food_color, food_square, context.transform, gl)
        });
    }

    pub fn set_max_coord(&mut self, max_coord: Coord) {
        self.max_coord = max_coord;
    }
}
