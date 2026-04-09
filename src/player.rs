use raylib::{RaylibHandle, RaylibThread, camera::Camera2D, ffi::KeyboardKey, math::Vector2};

use crate::{
    base::body::Body,
    engine::{Engine, Plugin},
};

#[derive(Debug, Default)]
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    #[inline(always)]
    fn add_plugin(engine: &mut Engine, rl: &mut RaylibHandle, thread: &RaylibThread) {
        engine.add_update(Player::update);
    }
}
#[derive(Debug, Default)]
pub struct Player {
    pub camera: bool,
}
impl Player {
    #[inline(always)]
    pub fn update(engine: &mut Engine, (rl, _): (&mut RaylibHandle, &mut RaylibThread), dt: f32) {
        {
            let mut target = Vector2::zero();
            for (player, body) in engine.world.query::<(&Player, &Body)>().iter() {
                target = body.pos;
            }
            let Some(camera) = engine.resource_mut::<Camera2D>() else {
                return;
            };
            camera.target = target
                - Vector2::new(rl.get_screen_width() as f32, rl.get_screen_height() as f32)
                    / 2.0
                    / camera.zoom
        }
        {
            for (player, body) in engine.world.query_mut::<(&Player, &mut Body)>() {
                let mut acc = Vector2::zero();
                if rl.is_key_down(KeyboardKey::KEY_W) {
                    let direction =
                        Vector2::new(body.rot.to_radians().cos(), body.rot.to_radians().sin());
                    acc += direction * 500.0;
                }
                if rl.is_key_down(KeyboardKey::KEY_D) {
                    body.torque += 300.0 * dt;
                }
                if rl.is_key_down(KeyboardKey::KEY_A) {
                    body.torque -= 300.0 * dt;
                }
                body.vel += acc * dt;
                const FRICTION: f32 = 0.002;
                if body.torque > FRICTION {
                    body.torque -= FRICTION;
                } else if body.torque < -FRICTION {
                    body.torque += FRICTION;
                } else {
                    body.torque = 0.0;
                }
            }
        }
    }
}
