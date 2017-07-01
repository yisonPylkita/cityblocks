extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("City Blocks", [640, 480])
    .exit_on_esc(true).build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.0, 1.0, 1.0, 1.0], g);
            rectangle([0.15, 0.69, 0.35, 1.0],
                      [220.0, 0.0, 200.0, 100.0],
                      c.transform, g);
        });
    }
}
