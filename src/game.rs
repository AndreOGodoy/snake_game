use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub struct Game {
    pub gl: GlGraphics,
}

impl Game {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |_context, gl| {
            graphics::clear([90.0 / 255.0, 177.0 / 255.0, 187.0 / 255.0, 1.0], gl);
        })
    }
}
