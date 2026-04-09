pub mod body;
pub mod render;

use crate::engine::Plugin;
use raylib::{RaylibHandle, RaylibThread};

#[derive(Debug, Default)]
pub struct BasePlugin;
impl Plugin for BasePlugin {
    fn add_plugin(
        engine: &mut crate::engine::Engine,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        engine.add_plugin::<body::BodyPlugin>(rl, thread);
        engine.add_plugin::<render::RenderPlugin>(rl, thread);
    }
}
