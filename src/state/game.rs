use ggez::{
    event::EventHandler,
    graphics::{self, Color, DrawParam, MeshBuilder, Rect},
    Context, GameResult,
};
use legion::*;
use rand::prelude::*;

use crate::{
    component::{Colour, Level, Player, ScreenDimensions, Star, Velocity},
    system::move_stars_system,
};

pub struct Game {
    world: World,
    schedule: Schedule,
}

fn generate_stars(num: usize, screen_coordinates: Rect) -> Vec<(Star, Level, Colour)> {
    let mut stars = Vec::with_capacity(num);

    let mut rng = rand::thread_rng();
    for _ in 0..num {
        let x: f32 = rng.gen_range(screen_coordinates.left(), screen_coordinates.right());
        let y: f32 = rng.gen_range(screen_coordinates.top(), screen_coordinates.bottom());
        let grey: f32 = rng.gen_range(0.3, 0.95);
        let level: f32 = rng.gen_range(0.0, 1.0);

        stars.push((
            Star::new(x, y),
            Level(level),
            Color::new(grey, grey, grey, 1.0),
        ));
    }

    return stars;
}

impl Game {
    pub fn new(ctx: &mut Context) -> Self {
        let mut world = World::default();

        world.extend(generate_stars(1024, graphics::screen_coordinates(ctx)));

        world.push((Player, Velocity::new(1.0, 1.0)));

        let schedule = Schedule::builder().add_system(move_stars_system()).build();

        Game { world, schedule }
    }

    fn draw_starfield(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut star_lines = &mut MeshBuilder::new();
        for (star, colour) in <(&Star, &Colour)>::query().iter(&mut self.world) {
            star_lines = star_lines.line(&[star.pos1, star.pos2], 1.0, *colour)?;
        }
        let mesh = star_lines.build(ctx)?;

        let params = DrawParam::default();

        graphics::draw(ctx, &mesh, params)
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let screen_dimensions = ScreenDimensions(graphics::screen_coordinates(ctx));

        let mut resources = Resources::default();
        resources.insert(screen_dimensions);

        self.schedule.execute(&mut self.world, &mut resources);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        self.draw_starfield(ctx)?;

        graphics::present(ctx)
    }
}
