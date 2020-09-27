use crate::component::{Level, ScreenDimensions, Star};
use legion::*;

#[system(for_each)]
pub fn move_stars(
    star: &mut Star,
    level: &Level,
    #[resource] screen_dimensions: &ScreenDimensions,
) {
    star.pos1.x += 1.0 * level.0;
    star.pos1.y += 1.0 * level.0;
}
