use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub struct Snake {
    pub x: f64,
    pub y: f64,
    tile_size: f64,
}

impl Snake {
    pub fn new(x: f64, y: f64, tile_size: f64) -> Self {
        Snake {x, y, tile_size}
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        let head = graphics::rectangle::square(self.tile_size * self.x, self.tile_size * self.y, 20.0);

        gl.draw(args.viewport(), |context, gl| {
            graphics::rectangle([1.0, 0.0, 1.0, 1.0], head, context.transform, gl)
        });
    }
}
