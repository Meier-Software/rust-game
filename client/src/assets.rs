use ggez::{Context, GameResult};
use ggez::graphics::Image;
use std::collections::HashMap;

pub struct Asset {
    pub name: String,
    pub img: Image,
}

impl Asset {
    pub fn new(name: String, img: Image) -> Self {
        Self { name, img }
    }
}

pub struct AssetManager {
    assets: HashMap<String, Asset>,
}

impl AssetManager {
    pub fn new() -> Self {
        Self {
            assets: HashMap::new(),
        }
    }

    pub fn load_asset(&mut self, ctx: &mut Context, name: &str, path: &str) -> GameResult<()> {
        let img = Image::from_path(ctx, path)?;
        let asset = Asset::new(name.to_string(), img);
        self.assets.insert(name.to_string(), asset);
        Ok(())
    }

    pub fn get_asset(&self, name: &str) -> Option<&Asset> {
        self.assets.get(name)
    }
}