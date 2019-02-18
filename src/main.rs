extern crate tetra;
use tetra::graphics::{self, Color, Texture, Vec2};
use tetra::{State, Context, ContextBuilder};

use std::rc::Rc;

struct Building {
    texture: Rc<Texture>,
    position: Vec2,
}

struct CityMapState {
    buildings: Vec<Building>,
}

impl CityMapState {
    fn new(ctx: &mut Context) -> tetra::Result<CityMapState> {
        let empty_space_texture = Texture::new(ctx, "./assets/city_map/empty_space.png")?;
        let mut buildings: Vec<Building> = Vec::new();
        buildings.push(Building {
            texture: empty_space_texture,
            position: Vec2::new(0.0, 0.0),
        });
        buildings.push(Building {
            texture: empty_space_texture,
            position: Vec2::new(0.2, 0.2),
        });

        Ok(CityMapState {
            buildings: buildings,
        })
    }
}

impl State for CityMapState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context, _dt: f64) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.15, 0.25, 0.35));
        for building in self.buildings.iter() {
            graphics::draw(ctx, &building.texture, building.position);
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("City Blocks - dev", 1280, 720)
        .build()?
        .run_with(CityMapState::new)
}
