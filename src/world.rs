use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use hecs::Entity;
use raylib::{
    RaylibHandle, RaylibThread,
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle},
};

use crate::engine::{Engine, Plugin};

pub struct GalaxyPlugin;
impl Plugin for GalaxyPlugin {
    #[inline(always)]
    fn add_plugin(engine: &mut Engine, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let seed = *engine
            .resource::<u32>()
            .expect("no seed (u32) in resources");
        engine
            .add_resource(Galaxy {
                seed,
                ..Default::default()
            })
            .add_draw(Galaxy::draw);
    }
}

#[derive(Debug, Default)]
pub struct Galaxy {
    seed: u32,
    // galactic -> solar -> planetary
    scale: Scale<u16, Scale<u16, Scale<u16, Space>>>,
}
impl Galaxy {
    #[inline(always)]
    pub fn get_space(
        &self,
        (gpos, spos, ppos): ((u16, u16), (u16, u16), (u16, u16)),
    ) -> Option<&Space> {
        self.scale
            .get(gpos)
            .and_then(|scale| scale.get(spos))
            .and_then(|scale| scale.get(ppos))
    }
    #[inline(always)]
    pub fn get_space_mut(
        &mut self,
        (gpos, spos, ppos): ((u16, u16), (u16, u16), (u16, u16)),
    ) -> Option<&mut Space> {
        self.scale
            .get_mut(gpos)
            .and_then(|scale| scale.get_mut(spos))
            .and_then(|scale| scale.get_mut(ppos))
    }
    #[inline(always)]
    pub fn draw(engine: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
        d.clear_background(Color::BLACK);
    }
}

#[derive(Debug, Default)]
pub struct Scale<S, T>
where
    S: Debug + Default + Eq + Hash,
    T: Debug + Default,
{
    chunks: HashMap<(S, S), T>,
}
impl<S, T> Scale<S, T>
where
    S: Debug + Default + Eq + Hash,
    T: Debug + Default,
{
    #[inline(always)]
    pub fn get(&self, pos: (S, S)) -> Option<&T> {
        self.chunks.get(&pos)
    }
    #[inline(always)]
    pub fn get_mut(&mut self, pos: (S, S)) -> Option<&mut T> {
        self.chunks.get_mut(&pos)
    }
}

#[derive(Debug, Default)]
pub struct Space {
    entities: HashSet<Entity>,
}
