extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{ButtonEvent, RenderEvent, UpdateEvent};
use piston::window::Window;
use piston::window::WindowSettings;
use piston::ButtonState;

mod game;
use game::Game;

mod coord;

fn main() {
    let opengl = OpenGL::V4_5;

    let (mut width, mut height): (f64, f64) = (800.0, 600.0);
    let tile_size = 40.0;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", (width, height))
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .expect("Coudn't open window");

    let mut game = Game::new(GlGraphics::new(opengl), tile_size, (width, height));

    let mut event = Events::new(EventSettings::new()).ups(10);

    while let Some(e) = event.next(&mut window) {
        if let Some(arg) = e.render_args() {
            game.render(&arg)
        }

        if let Some(_arg) = e.update_args() {
            width = window.size().width;
            height = window.size().height;
            game.set_max_size((height, width));
            game.update();
            if game.snake.is_in_body(game.snake.head_coord) {
                break;
            }
        }

        if let Some(arg) = e.button_args() {
            if arg.state == ButtonState::Press {
                game.register(&arg.button);
            }
        }
    }
}
