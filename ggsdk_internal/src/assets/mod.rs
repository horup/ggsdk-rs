mod loader;
use kira::sound::static_sound::StaticSoundData;
pub use loader::*;

#[cfg(not(target_arch = "wasm32"))]
mod disk_loader;
#[cfg(not(target_arch = "wasm32"))]
use disk_loader::DiskLoader;

#[cfg(target_arch = "wasm32")]
mod web_loader;
#[cfg(target_arch = "wasm32")]
pub use web_loader::*;

use std::{any::{Any, TypeId}, collections::HashMap, io::Cursor, ops::{Deref, DerefMut}, path::Path, rc::Rc, str::from_utf8};

use crate::{GGAtlas, GGContext};

#[derive(Clone)]
pub struct GGAsset<T> {
    pub name: String,
    pub path: String,
    pub data: T,
}

impl<T> Deref for GGAsset<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for GGAsset<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

pub enum AssetEvent<T> {
    Loaded(Rc<GGAsset<T>>),
    LoadFailed { name: String, path: String },
}

pub struct TypedAssets<T> {
    loaded: HashMap<String, Rc<GGAsset<T>>>,
    pending: HashMap<String, String>,
    loader: Box<dyn Loader>,
}

pub struct Load {
    pub name:String,
    pub path:String,
    pub data:Vec<u8>
}

impl<T : 'static> TypedAssets<T> {
    pub fn to_any(self) -> Box<dyn std::any::Any> {
        Box::new(self)
    }

    pub fn new(loader: impl Loader) -> Self {
        Self {
            loaded: Default::default(),
            pending: Default::default(),
            loader: Box::new(loader),
        }
    }
    pub fn load(&mut self, path: &str, name: &str) {
        self.pending.insert(path.to_string(), name.to_string());
        self.loader.request(path.into());
    }
   
    pub fn get(&self, name: &str) -> Option<Rc<GGAsset<T>>> {
        self.loaded.get(name).cloned()
    }
    pub fn poll<F>(&mut self, f: F) -> Option<AssetEvent<T>>
    where
        F: Fn(Load) -> Result<T, ()>,
    {
        if let Some(e) = self.loader.poll() {
            let e: Option<AssetEvent<T>> = match e {
                LoaderEvent::Load(path, vec) => match self.pending.remove(&path) {
                    Some(name) => match f(Load { name: name.clone(), path: path.clone(), data: vec }) {
                        Ok(t) => {
                            let asset = Rc::new(GGAsset {
                                name: name.clone(),
                                path,
                                data: t,
                            });
                            self.loaded.insert(name.clone(), asset.clone());
                            Some(AssetEvent::Loaded(asset))
                        }
                        Err(_) => Some(AssetEvent::LoadFailed { name, path }),
                    },
                    None => None,
                },
                LoaderEvent::LoadFailed(path) => self.pending.remove(&path).map(|name| AssetEvent::LoadFailed { name, path }),
            };

            return e;
        }

        None
    }
}

impl<T> Default for TypedAssets<T> {
    fn default() -> Self {
        #[cfg(target_arch = "wasm32")]
        type L = WebLoader;
        #[cfg(not(target_arch = "wasm32"))]
        type L = DiskLoader;
        let loader = Box::new(L::default());
        Self {
            loaded: Default::default(),
            pending: Default::default(),
            loader,
        }
    }
}

#[derive(Default)]
pub struct GAssets {
    assets: HashMap<TypeId, Box<dyn AssetLoader>>,
    total: usize,
    pending: usize,
}

pub trait AssetLoader {
    fn poll(&mut self, g:&mut GGContext) -> bool;
    fn to_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn to_any_ref(&self) -> &dyn std::any::Any;
}

impl AssetLoader for TypedAssets<String> {
    fn poll(&mut self, _:&mut GGContext) -> bool {
        self.poll(|l| match from_utf8(&l.data) {
            Ok(ok) => Ok(ok.to_string()),
            Err(_) => Err(()),
        })
        .is_some()
    }

    fn to_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn to_any_ref(&self) -> &dyn Any {
        self
    }
}

struct TiledMapReader {
    pub data: Vec<u8>,
}

impl tiled::ResourceReader for TiledMapReader {
    type Resource = Cursor<Vec<u8>>;
    type Error = std::io::Error;

    fn read_from(
        &mut self,
        _: &std::path::Path,
    ) -> std::result::Result<Self::Resource, Self::Error> {
        Ok(Cursor::new(self.data.clone()))
    }
}

impl AssetLoader for TypedAssets<tiled::Map> {
    fn poll(&mut self, _:&mut GGContext) -> bool {
        self.poll(|load|{
            let data = load.data.clone();
            let reader = TiledMapReader { data };
            let mut loader = tiled::Loader::with_reader(reader);
            let path = load.path.clone();
            let map = loader.load_tmx_map(path);
            match map {
                Ok(map) => Ok(map),
                Err(_) => Err(()),
            }
        }).is_some()
    }

    fn to_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn to_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}

impl AssetLoader for TypedAssets<GGAtlas> {
    fn poll(&mut self, g:&mut GGContext) -> bool {
        self.poll(|load|{
            let path = Path::new(&load.path);
            let file_name = path.file_stem().unwrap_or_default();
            let split = file_name.to_str().unwrap_or_default();
            let split = split.split("_");
            let last = split.last();
            let mut cols = 1;
            let mut rows = 1;
    
            if let Some(last) = last {
                let mut split = last.split("x");
                if let (Some(c), Some(r)) = (split.next(), split.next()) {
                    if let (Ok(c), Ok(r)) = (u8::from_str_radix(c, 10), u8::from_str_radix(r, 10)) {
                        cols = c;
                        rows = r;
                    }
                }
            }
            Ok(GGAtlas::new(g.egui_ctx, load.name, &load.data, cols, rows))
        }).is_some()
    }

    fn to_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn to_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}

impl AssetLoader for TypedAssets<StaticSoundData> {
    fn poll(&mut self, _:&mut GGContext) -> bool {
        self.poll(|load| {
            let data = load.data.clone();
            let cursor = std::io::Cursor::new(data);
            let cursor = StaticSoundData::from_cursor(cursor);
            match cursor {
                Ok(sound_data) => Ok(sound_data),
                Err(_) => Err(()),
            }
        })
        .is_some()
    }

    fn to_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn to_any_ref(&self) -> &dyn std::any::Any {
        self
    }
}

impl GAssets {
    pub fn load<T: 'static + Clone>(&mut self, path: &str, name: &str)
    where
        TypedAssets<T>: AssetLoader,
    {
        let type_id = TypeId::of::<T>();
        let assets = match self.assets.get_mut(&type_id) {
            Some(assets) => assets,
            None => {
                let assets: TypedAssets<T> = TypedAssets::default();
                self.assets.insert(type_id, Box::new(assets));
                self.assets.get_mut(&type_id).unwrap()
            }
        };
        let assets: &mut dyn Any = assets.to_any_mut();
        let Some(assets) = assets.downcast_mut::<TypedAssets<T>>() else {
            return;
        };
        assets.load(path, name);
        self.pending += 1;
        self.total += 1;
    }

    pub fn get<T: 'static + Clone>(&self, name: &str) -> Option<Rc<GGAsset<T>>> {
        let assets = self.assets.get(&TypeId::of::<T>());
        let Some(assets) = assets else {
            return None;
        };
        let assets: &dyn Any = assets.to_any_ref();
        let assets: Option<&TypedAssets<T>> = assets.downcast_ref();
        let Some(assets) = assets else { return None };
        assets.get(name)
    }

    pub fn poll(&mut self, g:&mut GGContext) {
        for assets in self.assets.values_mut() {
            if assets.poll(g) {
                self.pending -= 1;
            }
        }
    }

    pub fn pending(&self) -> usize {
        self.pending
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn loaded(&self) -> usize {
        self.total - self.pending
    }
}