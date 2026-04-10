pub mod planet;
pub mod player;
pub mod world;

use crate::engine::Plugin;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, Default)]
pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn add_plugin(
        engine: &mut crate::engine::Engine,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        engine.add_plugin::<world::GalaxyPlugin>(rl, thread);
        engine.add_plugin::<player::PlayerPlugin>(rl, thread);
        engine.add_plugin::<planet::PlanetPlugin>(rl, thread);
    }
}
