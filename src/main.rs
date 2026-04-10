#![allow(unused)]
pub mod base;
pub mod engine;
pub mod game;
pub mod gui;

use std::f32::consts::PI;

use crate::{
    base::{
        BasePlugin,
        body::{Body, BodyProps, CollisionShape},
        render::{Asset, RenderPlugin},
    },
    engine::{Engine, EngineConfig},
    game::{
        GamePlugin,
        planet::{Atmosphere, Planet},
        player::Player,
    },
    gui::GUIPlugin,
};
use hecs::DynamicBundle;
use raylib::prelude::*;

fn main() {
    // start
    let mut engine = Engine::default();
    let (mut rl, mut thread) = engine.init(EngineConfig {
        title: "Orbita",
        w: (1920.0 / 1.5) as i32,
        h: (1080.0 / 1.5) as i32,
        ..Default::default()
    });
    // setup
    engine
        .add_resource(Camera2D {
            zoom: 1.0 / 8.0,
            ..Default::default()
        })
        .add_resource(42u32)
        .add_plugin::<BasePlugin>(&mut rl, &thread)
        .add_plugin::<GamePlugin>(&mut rl, &thread)
        .add_plugin::<GUIPlugin>(&mut rl, &thread);
    // setup spawn
    engine.spawn(comp_player());
    engine.spawn(comp_earth());
    // engine.spawn(comp_meteor(
    //     Vector2 { x: 1.2, y: 1.0 }.normalized() * 800.0,
    //     Vector2::zero(),
    //     0.0,
    //     0.0,
    // ));
    // run
    engine.run(&mut rl, &mut thread);
}

pub fn comp_player() -> impl DynamicBundle {
    (
        Body {
            pos: Vector2::one() * 10000.0,
            vel: Vector2::zero(),
            rot: 0.0,
            torque: 0.0,
            shape: CollisionShape::Circle(100.0),
            properties: BodyProps::default(),
            parent: None,
        },
        Asset {
            path: "Ships/spaceShips_009.png",
            scale: Vector2::one(),
            rot_offset: -90.0,
        },
        Player { camera: true },
    )
}
pub fn comp_earth() -> impl DynamicBundle {
    (
        Body {
            pos: Vector2::default(),
            vel: Vector2::zero(),
            rot: 0.0,
            torque: 0.0,
            shape: CollisionShape::Circle(10000.0),
            properties: BodyProps {
                mass: 500.0,
                ..Default::default()
            },
            parent: None,
        },
        Planet {
            color: Color::GREEN,
            atmosphere: Some(Atmosphere {
                color: Color::SKYBLUE.alpha(0.5),
                height: 800.0,
                friction: 200.0,
            }),
        },
    )
}
pub fn comp_meteor(pos: Vector2, vel: Vector2, rot: f32, torque: f32) -> impl DynamicBundle {
    (
        Body {
            pos,
            vel,
            rot,
            torque,
            shape: CollisionShape::Circle(200.0),
            properties: BodyProps {
                mass: 8.0,
                ..Default::default()
            },
            parent: None,
        },
        Asset {
            path: "Meteors/spaceMeteors_001.png",
            scale: Vector2::one(),
            rot_offset: -90.0,
        },
    )
}
