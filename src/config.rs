use ggez::conf::{FullscreenType, WindowMode};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    window_dimensions: (usize, usize),
    fullscreen: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            window_dimensions: (1280, 720),
            fullscreen: false,
        }
    }
}

impl Config {
    pub fn to_window_mode(&self) -> WindowMode {
        WindowMode::default()
            .dimensions(
                self.window_dimensions.0 as f32,
                self.window_dimensions.1 as f32,
            )
            .borderless(self.fullscreen)
            .fullscreen_type(if self.fullscreen {
                FullscreenType::True
            } else {
                FullscreenType::Windowed
            })
    }
}
