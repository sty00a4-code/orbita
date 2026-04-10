use std::collections::{HashMap, HashSet};

use crate::{
    base::body::{Body, CollisionShape},
    engine::{Engine, Plugin},
};
use hecs::Entity;
use raylib::{
    RaylibHandle, RaylibThread,
    color::Color,
    math::Vector2,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

#[derive(Debug, Default)]
pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn add_plugin(
        engine: &mut crate::engine::Engine,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        engine.add_update(Planet::update).add_draw(Planet::draw);
    }
}

#[derive(Debug, Default)]
pub struct Planet {
    pub color: Color,
    pub atmosphere: Option<Atmosphere>,
}
#[derive(Debug, Default)]
pub struct Atmosphere {
    pub color: Color,
    pub height: f32,
    pub friction: f32,
}
impl Planet {
    pub fn update(engine: &mut Engine, _: (&mut RaylibHandle, &mut RaylibThread), dt: f32) {
        let mut planets: HashSet<Entity> = HashSet::default();
        for (e, p, body) in engine.world.query_mut::<(Entity, &Planet, &mut Body)>() {
            body.vel = Vector2::zero();
            planets.insert(e);
        }
        let mut bodies = vec![];
        for (e_b, body) in engine.world.query_mut::<(Entity, &mut Body)>() {
            if planets.contains(&e_b) {
                continue;
            }
            bodies.push(e_b);
        }
        for p_e in planets {
            for b_e in bodies.iter() {
                if let Ok(planet) = engine.world.get::<&Planet>(p_e)
                    && let Ok(planet_body) = engine.world.get::<&Body>(p_e)
                    && let Ok(mut body) = engine.world.get::<&mut Body>(*b_e)
                    && let Some(Atmosphere {
                        height, friction, ..
                    }) = planet.atmosphere
                    && let CollisionShape::Circle(planet_rad) = planet_body.shape
                    && let CollisionShape::Circle(body_rad) = body.shape
                    && planet_body.pos.distance_to(body.pos) < planet_rad + height + body_rad
                {
                    let dir = body.vel.normalized();
                    body.vel -= dir * friction * dt;
                }
            }
        }
    }
    pub fn draw(engine: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
        for (planet, body) in engine.world.query::<(&Planet, &Body)>().iter() {
            if let CollisionShape::Circle(rad) = body.shape {
                if let Some(atmo) = &planet.atmosphere {
                    d.draw_circle_v(body.pos, rad + atmo.height, atmo.color);
                }
                d.draw_circle_v(body.pos, rad, planet.color);
            }
        }
    }
}
