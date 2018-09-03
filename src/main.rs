extern crate ggez;
use ggez::*;
use std::env;
use std::path;
use ggez::graphics::{DrawMode, Point2, MeshBuilder, Mesh};


const SIZE: f32 = 600.0;
const CELL_NUMBER: f32 = 15.0;
const GRID_SIZE: f32  = SIZE / CELL_NUMBER;


fn background(ctx: &mut Context) -> Mesh{
    let mesh = &mut MeshBuilder::new();
    
    for i in 0..CELL_NUMBER as u32{
        for j in 0..CELL_NUMBER as u32{
            let start_x = i as f32 * GRID_SIZE;
            let start_y = j as f32 * GRID_SIZE;

            mesh.polygon(graphics::DrawMode::Line(1.0),             
            &[
                Point2::new(start_x, start_y),
                Point2::new(GRID_SIZE, start_y),
                Point2::new(GRID_SIZE, GRID_SIZE),
                Point2::new(start_x, GRID_SIZE),
                Point2::new(start_x, start_y),
            ],);
        }

    }

    mesh.build(ctx).unwrap()

}

struct State {
    dt: std::time::Duration,
    bk: Mesh
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let mesh = background(ctx);
        let s = State { dt: std::time::Duration::new(0, 0), bk: mesh };
        graphics::set_color(ctx, [0.0, 0.0, 1.0, 1.0].into());
        Ok(s)
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.dt = timer::get_delta(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, graphics::BLACK);
        graphics::clear(ctx);

        let fps = (1.0 / (self.dt.as_secs() as f64 + self.dt.subsec_nanos() as f64 * 1e-9)) as i32;
        let font = graphics::Font::new(ctx, "/font.ttf", 8)?;
        let text = graphics::Text::new(ctx, & fps.to_string(), &font)?;

        graphics::draw(ctx, &self.bk, Point2::new(10.0, 10.0), 0.0).unwrap();
        
        // Drawables are drawn from their top-left corner.
        let dest_point = graphics::Point2::new(SIZE + 10.0 , 10.0);
        graphics::draw(ctx, &text, dest_point, 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let config = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("snake", "Jarod", config).unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut State::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}