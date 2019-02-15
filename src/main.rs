extern crate tetra;
use tetra::graphics::{self, Color, Text, Font, Vec2};
use tetra::{State, Context, ContextBuilder};

use std::time::SystemTime;

struct GameState {
    text: Text,
    position: Vec2,
    last_frame_time: SystemTime,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            text: Text::new("City Blocks (dev)", Font::default(), 16.0),
            position: Vec2::new(10.0, 25.0),
            last_frame_time: SystemTime::now(),
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0., 0., 0.));
        graphics::draw(ctx, &self.text, self.position);

        let time_now = SystemTime::now();
        println!("Last frame was rendered {:?}ms ago", time_now.duration_since(self.last_frame_time).unwrap());
        self.last_frame_time = time_now;
        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("My First Tetra Game", 1280, 720)
        .build()?
        .run(&mut GameState::new())
}
