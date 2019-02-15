extern crate tetra;
use tetra::graphics::{self, Color, Text, Font, Vec2};
use tetra::{State, Context, ContextBuilder};

struct GameState {
    text: Text,
    position: Vec2,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            text: Text::new("Hello, world!", Font::default(), 16.0),
            position: Vec2::new(0.0, 0.0),
        }
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.position.x += 1.0;

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));
        graphics::draw(ctx, &self.text, self.position);

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("My First Tetra Game", 1280, 720)
        .build()?
        .run(&mut GameState::new())
}
