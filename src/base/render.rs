use crate::base::body::Body;
use crate::engine::{Engine, Plugin};
use raylib::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Default)]
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    #[inline(always)]
    fn add_plugin(
        engine: &mut crate::engine::Engine,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
    ) {
        engine
            .add_resource(AssetServer::load(rl, thread))
            .add_draw(Asset::draw);
    }
}

#[derive(Debug, Default)]
pub struct AssetServer {
    pub assets: HashMap<String, Texture2D>,
    paths: HashMap<String, String>,
}
impl AssetServer {
    #[inline(always)]
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let mut paths = HashMap::new();

        fn load_dir(dir: &Path, paths: &mut HashMap<String, String>, base: &Path) {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        load_dir(&path, paths, base);
                    } else if path.extension().and_then(|s| s.to_str()) == Some("png")
                        && let Ok(relative) = path.strip_prefix(base)
                        && let Some(key) = relative.to_str()
                        && let Some(path_str) = path.to_str()
                    {
                        paths.insert(key.to_string(), path_str.to_string());
                    }
                }
            }
        }

        let assets_path = Path::new("assets");
        load_dir(assets_path, &mut paths, assets_path);

        let mut server = Self {
            assets: HashMap::new(),
            paths,
        };
        server.load_textures(rl, thread);
        server
    }

    #[inline(always)]
    pub fn load_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        for (key, path) in &self.paths {
            if let Ok(texture) = rl.load_texture(thread, path) {
                self.assets.insert(key.clone(), texture);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    pub path: &'static str,
    pub scale: Vector2,
    pub rot_offset: f32,
}
impl Asset {
    #[inline(always)]
    pub fn draw(engine: &mut Engine, (d, _): (&mut RaylibDrawHandle, &mut RaylibThread)) {
        let Some(asset_server) = engine.resource::<AssetServer>() else {
            return;
        };
        for (body, asset) in engine.world.query::<(&Body, &Asset)>().iter() {
            let Some(texture) = asset_server.assets.get(asset.path) else {
                continue;
            };
            let source_rect =
                Rectangle::new(0.0, 0.0, texture.width() as f32, texture.height() as f32);
            let dest_rect = Rectangle::new(
                body.pos.x,
                body.pos.y,
                texture.width() as f32 * asset.scale.x,
                texture.height() as f32 * asset.scale.y,
            );
            d.draw_texture_pro(
                texture,
                source_rect,
                dest_rect,
                Vector2::new(
                    texture.width() as f32 * asset.scale.x / 2.0,
                    texture.height() as f32 * asset.scale.y / 2.0,
                ),
                body.rot + asset.rot_offset,
                Color::WHITE,
            );
        }
    }
}
