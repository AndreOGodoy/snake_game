extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sprite;

use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{ButtonEvent, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod game;
use game::Game;

fn main() {
    let opengl = OpenGL::V4_5;

    let mut window: GlutinWindow = WindowSettings::new("Snake Game", [800, 600])
        .exit_on_esc(true)
        .build()
        .expect("Coudn't open window");

    let mut game = Game {
        gl: GlGraphics::new(opengl),
    };

    let mut event = Events::new(EventSettings::new());
    while let Some(e) = event.next(&mut window) {
        if let Some(arg) = e.render_args() {
            game.render(&arg)
        }

        if let Some(arg) = e.update_args() {
            //Do something here
        }

        if let Some(arg) = e.button_args() {
            //Do something here
        }
    }
}
