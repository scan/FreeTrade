use ggez::{graphics::Color, nalgebra::Point2};

pub type Position = Point2<f32>;
pub type Velocity = Point2<f32>;
pub type Colour = Color;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Level(pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Star {
    pub pos1: Point2<f32>,
    pub pos2: Point2<f32>,
}

impl Star {
    pub fn new(x: f32, y: f32) -> Self {
        Star {
            pos1: Point2::new(x, y),
            pos2: Point2::new(x + 1.0, y),
        }
    }
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Player;
