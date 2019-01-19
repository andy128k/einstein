#![macro_use]

use std::collections::HashMap;
use std::marker::PhantomData;
use std::cell::{RefCell, Ref};
use sdl2::surface::Surface;
use sdl2::render::{TextureCreator, Texture};
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::mixer::{Chunk, LoaderRWops};

pub struct Resource {
    pub name: &'static str,
    pub data: &'static [u8],
}

#[macro_export]
macro_rules! resource {
    ( $path:expr ) => {
        crate::resources::manager::Resource {
            name: $path,
            data: include_bytes!($path),
        }
    };
}

pub trait ResourceManager {
    fn image(&self, resource: &'static Resource, highlighted: bool) -> Ref<Texture>;
    fn font(&self, point_size: u16) -> Ref<Font>;
    fn chunk(&self, resource: &'static Resource) -> Ref<Chunk>;
}

const FONT_DUMP: &[u8] = include_bytes!("./fonts/LiberationSans-Regular.ttf"); // /usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf

pub struct ResourceManagerImpl<'r, C> where C: 'r{
    images: RefCell<HashMap<String, Texture<'r>>>,
    fonts: RefCell<HashMap<u16, Font<'r, 'r>>>,
    chunks: RefCell<HashMap<String, Chunk>>,
    texture_creator: &'r TextureCreator<C>,
    ttf_context: &'r Sdl2TtfContext,
    phantom_data: PhantomData<&'r str>,
}

impl<'r, C> ResourceManagerImpl<'r, C> where C: 'r {
    pub fn new(texture_creator: &'r TextureCreator<C>, ttf_context: &'r Sdl2TtfContext) -> Self {
        ResourceManagerImpl {
            images: RefCell::new(HashMap::new()),
            fonts: RefCell::new(HashMap::new()),
            chunks: RefCell::new(HashMap::new()),
            texture_creator,
            ttf_context,
            phantom_data: PhantomData,
        }
    }

    fn load_image(&self, data: &[u8]) -> Result<Texture<'r>, ::failure::Error> {
        let mut rw = RWops::from_bytes(data).map_err(::failure::err_msg)?;
        let surface = Surface::load_bmp_rw(&mut rw).map_err(::failure::err_msg)?;
        let texture = self.texture_creator.create_texture_from_surface(surface)?;
        Ok(texture)
    }

    fn image_normal(&self, name: &'static str, data: &[u8]) -> Ref<Texture<'r>> {
        let key = name.to_owned();
        if self.images.borrow().get(&key).is_none() {
            let mut img = self.load_image(data).unwrap();
            img.set_color_mod(230, 230, 230); // TODO: remove this hack
            self.images.borrow_mut().insert(key.clone(), img);
        }
        Ref::map(self.images.borrow(), |r| r.get(&key).unwrap())
    }

    fn image_highlighted(&self, name: &'static str, data: &[u8]) -> Ref<Texture> {
        let key = format!("{}_HIGHLIGHTED", name);
        if self.images.borrow().get(&key).is_none() {
            let img = self.load_image(data).unwrap();
            self.images.borrow_mut().insert(key.clone(), img);
        }
        Ref::map(self.images.borrow(), |r| r.get(&key).unwrap())
    }
}

impl<'r, C> ResourceManager for ResourceManagerImpl<'r, C> {
    fn image(&self, resource: &'static Resource, highlighted: bool) -> Ref<Texture> {
        if highlighted {
            self.image_highlighted(resource.name, resource.data)
        } else {
            self.image_normal(resource.name, resource.data)
        }
    }

    fn font(&self, point_size: u16) -> Ref<Font> {
        if self.fonts.borrow().get(&point_size).is_none() {
            let ops = RWops::from_bytes(FONT_DUMP).unwrap();
            let font = self.ttf_context.load_font_from_rwops(ops, point_size).unwrap();
            self.fonts.borrow_mut().insert(point_size, font);
        }
        Ref::map(self.fonts.borrow(), |r| r.get(&point_size).unwrap())
    }

    fn chunk(&self, resource: &'static Resource) -> Ref<Chunk> {
        if self.chunks.borrow().get(resource.name).is_none() {
            let chunk = RWops::from_bytes(resource.data).unwrap().load_wav().unwrap();
            self.chunks.borrow_mut().insert(resource.name.to_owned(), chunk);
        }
        Ref::map(self.chunks.borrow(), |r| r.get(resource.name).unwrap())
    }
}
