use ggez::{
    conf::{NumSamples, WindowSetup},
    event, ContextBuilder, GameResult,
};

mod component;
mod config;
// mod resources;
mod state;
mod system;

fn main() -> GameResult {
    env_logger::init();

    let cfg = config::Config::default();

    let (mut ctx, mut event_loop) = ContextBuilder::new("Project: FreeTrade", "Lanbo")
        .window_mode(cfg.to_window_mode())
        .window_setup(
            WindowSetup::default()
                .title("Project: FreeTrade")
                .samples(NumSamples::Sixteen),
        )
        .add_resource_path("./resources")
        .build()?;

    let mut game = state::Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => log::info!("clean exit"),
        Err(e) => log::error!("error running game: {}", e),
    }

    Ok(())
}
