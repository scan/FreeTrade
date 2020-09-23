use ggez::{event, ContextBuilder, GameResult};

mod component;
mod state;

fn main() -> GameResult {
    env_logger::init();

    let (mut ctx, mut event_loop) = ContextBuilder::new("Project: FreeTrade", "Lanbo").build()?;

    let mut game = state::Game::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut game) {
        Ok(_) => log::info!("clean exit"),
        Err(e) => log::error!("error running game: {}", e),
    }

    Ok(())
}
