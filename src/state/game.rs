use ggez::{
    event::EventHandler,
    graphics::{self, DrawParam, MeshBuilder},
    Context, GameResult,
};
use legion::*;

use crate::component::{Colour, Position, Star};

pub struct Game {
    world: World,
}

fn generate_stars(num: usize) -> Vec<(Star, Colour, Position)> {
    let mut stars = Vec::with_capacity(num);

    for _ in 0..num {
        stars.push((Star::default(), graphics::WHITE, Position::new(10.0, 20.0)));
    }

    return stars;
}

impl Game {
    pub fn new(_ctx: &mut Context) -> Self {
        let mut world = World::default();

        world.extend(generate_stars(10));

        Game { world }
    }

    fn draw_starfield(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut star_lines = &mut MeshBuilder::new();
        for (_, colour, position) in <(&Star, &Colour, &Position)>::query().iter(&mut self.world) {
            star_lines = star_lines.line(&[*position, *position], 1.0, *colour)?;
        }
        let mesh = star_lines.build(ctx)?;

        let params = DrawParam::default();

        graphics::draw(ctx, &mesh, params)
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.draw_starfield(ctx)?;

        graphics::present(ctx)
    }
}
