#![allow(dead_code)]
use crate::base::render::AssetServer;
use hecs::{DynamicBundle, Entity, NoSuchEntity, World};
use raylib::{RaylibHandle, RaylibThread, camera::Camera2D, prelude::*};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[derive(Debug, Clone, PartialEq)]
pub struct EngineConfig {
    pub w: i32,
    pub h: i32,
    pub title: &'static str,
    pub fullscreen: bool,
    pub resizable: bool,
}
impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            w: 640,
            h: 480,
            title: "Game",
            fullscreen: false,
            resizable: false,
        }
    }
}

#[derive(Default)]
pub struct Engine {
    pub world: World,
    pub systems: Systems,
    pub resources: Resources,
    pub event_bus: EventBus,
}
impl Engine {
    pub fn init(&mut self, config: EngineConfig) -> (RaylibHandle, RaylibThread) {
        let mut builder = raylib::init();
        builder.size(config.w, config.h).title(config.title);
        if config.fullscreen {
            builder.fullscreen();
        }
        if config.resizable {
            builder.resizable();
        }
        builder.build()
    }
    pub fn run(&mut self, rl: &mut RaylibHandle, thread: &mut RaylibThread) {
        if let Some(asset_server) = self.resources.get_mut::<AssetServer>() {
            asset_server.load_textures(rl, thread);
        }
        // startup
        let startups = std::mem::take(&mut self.systems.startups);
        for startup in &startups {
            startup(self, (rl, thread));
        }
        self.systems.startups = startups;
        // loop
        while !rl.window_should_close() {
            // update
            let dt = rl.get_frame_time();
            let updates = std::mem::take(&mut self.systems.updates);
            for update in &updates {
                update(self, (rl, thread), dt);
            }
            self.systems.updates = updates;
            // draw
            let mut d = rl.begin_drawing(thread);
            let draws = std::mem::take(&mut self.systems.draws);
            if let Some(camera) = self.resource::<Camera2D>() {
                let mut d = d.begin_mode2D(camera);
                for draw in &draws {
                    draw(self, (&mut d, thread));
                }
            } else {
                for draw in &draws {
                    draw(self, (&mut d, thread));
                }
            }
            self.systems.draws = draws;
        }
    }
}
#[derive(Default)]
pub struct Systems {
    pub startups: Vec<StartupFn>,
    pub updates: Vec<UpdateFn>,
    pub draws: Vec<DrawFn>,
}
pub type StartupFn = fn(&mut Engine, (&mut RaylibHandle, &mut RaylibThread));
pub type UpdateFn = fn(&mut Engine, (&mut RaylibHandle, &mut RaylibThread), f32);
pub type DrawFn = fn(&mut Engine, (&mut RaylibDrawHandle, &mut RaylibThread));
#[derive(Default)]
pub struct Resources {
    map: HashMap<TypeId, Box<dyn Any>>,
}
impl Resources {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    #[inline(always)]
    /// Insert a resource of type `T`. Overwrites any previous value of the same type.
    pub fn insert<T: Any + 'static>(&mut self, resource: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(resource));
    }

    #[inline(always)]
    /// Get an immutable reference to the resource of type `T`, if present.
    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
    }

    #[inline(always)]
    /// Get a mutable reference to the resource of type `T`, if present.
    pub fn get_mut<T: Any + 'static>(&mut self) -> Option<&mut T> {
        self.map
            .get_mut(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_mut::<T>())
    }

    #[inline(always)]
    /// Remove and return the boxed resource (if you need ownership).
    pub fn remove<T: Any + 'static>(&mut self) -> Option<Box<dyn Any>> {
        self.map.remove(&TypeId::of::<T>())
    }
}

impl Engine {
    #[inline(always)]
    pub fn spawn(&mut self, components: impl DynamicBundle) -> Entity {
        self.world.spawn(components)
    }
    #[inline(always)]
    pub fn despawn(&mut self, entity: Entity) -> Result<(), NoSuchEntity> {
        self.world.despawn(entity)
    }
    #[inline(always)]
    pub fn add_startup(&mut self, system: StartupFn) -> &mut Self {
        self.systems.startups.push(system);
        self
    }
    #[inline(always)]
    pub fn add_update(&mut self, system: UpdateFn) -> &mut Self {
        self.systems.updates.push(system);
        self
    }
    #[inline(always)]
    pub fn add_draw(&mut self, system: DrawFn) -> &mut Self {
        self.systems.draws.push(system);
        self
    }
    #[inline(always)]
    pub fn add_plugin<P: Plugin>(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) -> &mut Self {
        P::add_plugin(self, rl, thread);
        self
    }
    #[inline(always)]
    pub fn add_resource<U: Any + 'static>(&mut self, resource: U) -> &mut Self {
        self.resources.insert::<U>(resource);
        self
    }
    #[inline(always)]
    pub fn resource<U: Any + 'static>(&self) -> Option<&U> {
        self.resources.get::<U>()
    }
    #[inline(always)]
    pub fn resource_mut<U: Any + 'static>(&mut self) -> Option<&mut U> {
        self.resources.get_mut::<U>()
    }
}

pub type SubscriptionId = u64;

pub type Subscriber = (SubscriptionId, Box<dyn Fn(&dyn Any) + Send + Sync>);
pub type Event = dyn Any + Send + Sync;
/// A simple observer/event-bus that stores events as `Any` and allows typed subscriptions.
pub struct EventBus {
    // map event type -> list of (subscription id, callback)
    listeners: HashMap<TypeId, Vec<Subscriber>>,
    // queued events produced this frame (or by poller)
    queue: Vec<Box<Event>>,
    next_id: SubscriptionId,
}

impl Default for EventBus {
    fn default() -> Self {
        Self {
            listeners: HashMap::new(),
            queue: Vec::new(),
            next_id: 1,
        }
    }
}

impl EventBus {
    #[inline(always)]
    pub fn subscribe<E, F>(&mut self, f: F) -> SubscriptionId
    where
        E: Any + Send + Sync + 'static,
        F: Fn(&E) + Send + Sync + 'static,
    {
        let tid = TypeId::of::<E>();
        let id = self.next_id;
        self.next_id += 1;

        let wrapper = Box::new(move |e: &dyn Any| {
            if let Some(typed) = e.downcast_ref::<E>() {
                f(typed);
            }
        }) as Box<dyn Fn(&dyn Any) + Send + Sync>;

        self.listeners.entry(tid).or_default().push((id, wrapper));
        id
    }
    #[inline(always)]
    pub fn unsubscribe<E>(&mut self, subscription_id: SubscriptionId) -> bool
    where
        E: Any + 'static,
    {
        let tid = TypeId::of::<E>();
        if let Some(vec) = self.listeners.get_mut(&tid) {
            let before = vec.len();
            vec.retain(|(id, _)| *id != subscription_id);
            return vec.len() != before;
        }
        false
    }
    #[inline(always)]
    pub fn emit<E>(&mut self, event: E)
    where
        E: Any + Send + Sync + 'static,
    {
        self.queue.push(Box::new(event));
    }

    #[inline(always)]
    pub fn dispatch(&mut self) {
        let mut drained = Vec::new();
        std::mem::swap(&mut drained, &mut self.queue);
        for boxed in drained {
            let tid = (*boxed).type_id();
            if let Some(list) = self.listeners.get(&tid) {
                for (_, cb) in list.iter() {
                    cb(&*boxed);
                }
            }
        }
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.listeners.clear();
    }
}

pub trait Plugin {
    fn add_plugin(engine: &mut Engine, rl: &mut RaylibHandle, thread: &RaylibThread);
}
