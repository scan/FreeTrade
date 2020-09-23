use ggez::{graphics::Color, nalgebra::Point2};

pub type Position = Point2<f32>;
pub type Velocity = Point2<f32>;
pub type Colour = Color;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Star;

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq)]
pub struct Player;
