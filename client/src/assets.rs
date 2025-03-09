use ggez::graphics::Image;
use ggez::{Context, GameResult};
use std::collections::HashMap;

pub struct Asset {
    pub img: Image,
}

impl Asset {
    pub fn new(img: Image) -> Self {
        Self { img }
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
        log::debug!("Loading asset '{}' from path '{}'", name, path);
        
        let img = match Image::from_path(ctx, path) {
            Ok(img) => {
                log::debug!("Successfully loaded image for '{}', dimensions: {}x{}", name, img.width(), img.height());
                img
            },
            Err(e) => {
                // Try alternative path if the first one fails
                let alt_path = if path.starts_with("/") {
                    path.strip_prefix("/").unwrap_or(path)
                } else {
                    path
                };

                log::warn!(
                    "Failed to load asset from {}: {}. Trying {}",
                    path, e, alt_path
                );
                
                match Image::from_path(ctx, alt_path) {
                    Ok(img) => {
                        log::debug!("Successfully loaded image from alternative path for '{}', dimensions: {}x{}", 
                                   name, img.width(), img.height());
                        img
                    },
                    Err(e) => {
                        log::error!("Failed to load asset from alternative path {}: {}", alt_path, e);
                        return Err(e);
                    }
                }
            }
        };

        let asset = Asset::new(img);
        self.assets.insert(name.to_string(), asset);
        log::debug!("Asset '{}' successfully added to asset manager", name);
        Ok(())
    }

    pub fn load_assets(&mut self, ctx: &mut Context, assets: &[(&str, &str)]) -> GameResult<()> {
        for (name, path) in assets {
            if let Err(e) = self.load_asset(ctx, name, path) {
                println!("Failed to load asset {}: {}", name, e);
                return Err(e);
            }
        }
        Ok(())
    }

    pub fn get_asset(&self, name: &str) -> Option<&Asset> {
        self.assets.get(name)
    }

    #[allow(unused)]
    pub fn has_asset(&self, name: &str) -> bool {
        self.assets.contains_key(name)
    }

    pub fn debug_print_loaded_assets(&self) {
        log::debug!("=== Loaded Assets ===");
        for name in self.assets.keys() {
            log::debug!("Asset: {}", name);
        }
        log::debug!("====================");
    }
}
