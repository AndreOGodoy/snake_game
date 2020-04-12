extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{ButtonEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::ButtonState;

mod game;
use game::Game;

mod coord;

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [800, 600])
        .exit_on_esc(true)
        .graphics_api(opengl)
        .build()
        .expect("Coudn't open window");

    let mut game = Game::new(40, GlGraphics::new(opengl));

    let mut event = Events::new(EventSettings::new()).ups(10);

    while let Some(e) = event.next(&mut window) {
        if let Some(arg) = e.render_args() {
            game.render(&arg)
        }

        if let Some(_arg) = e.update_args() {
            game.update()
        }

        if let Some(arg) = e.button_args() {
            if arg.state == ButtonState::Press {
                game.register(&arg.button);
            }
        }
    }
}
