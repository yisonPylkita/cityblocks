extern crate tetra;
use tetra::graphics::{self, Color, Text, Font, Vec2};
use tetra::{State, Context, ContextBuilder};


// TODO: draw menu
struct MenuState {
    text: Text,
    position: Vec2,
}

impl MenuState {
    fn new() -> MenuState {
        MenuState {
            text: Text::new("City Blocks (dev)", Font::default(), 16.0),
            position: Vec2::new(10.0, 25.0),
        }
    }
}

impl State for MenuState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0., 0., 0.));
        graphics::draw(ctx, &self.text, self.position);

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("My First Tetra Game", 1280, 720)
        .build()?
        .run(&mut MenuState::new())
}
