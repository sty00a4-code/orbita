use crate::{
    base::body::Body,
    engine::{Engine, Plugin},
    game::player::Player,
};
use raylib::prelude::*;

#[derive(Debug, Default)]
pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn add_plugin(
        engine: &mut crate::engine::Engine,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        engine.add_draw(HUD::draw);
    }
}

pub struct HUD;
impl HUD {
    pub fn draw(engine: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
        let Some(camera) = engine.resource::<Camera2D>() else {
            return;
        };
        let mut text = String::new();
        for (_, body) in engine.world.query::<(&Player, &Body)>().iter() {
            text = format!("{:09.2}", body.vel.length());
        }
        d.draw_text(
            &text,
            camera.target.x as i32,
            camera.target.y as i32,
            (26.0 / camera.zoom) as i32,
            Color::WHITE,
        );
    }
}
