use std::collections::HashMap;
use std::marker::PhantomData;
use std::cell::{RefCell, Ref};
use sdl::video::Surface;
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};
use ui::utils::{load_image, adjust_brightness};

pub trait ResourceManager {
    fn image(&self, name: &'static str, data: &[u8], highlighted: bool) -> Ref<Surface>;
    fn font(&self, point_size: u16) -> Ref<Font>;
}

const FONT_DUMP: &[u8] = include_bytes!("./fonts/LiberationSans-Regular.ttf"); // /usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf

pub struct ResourceManagerImpl<'r> {
    images: RefCell<HashMap<String, Surface>>,
    fonts: RefCell<HashMap<u16, Font<'r, 'r>>>,
    ttf_context: &'r Sdl2TtfContext,
    phantom_data: PhantomData<&'r str>,
}

impl<'r> ResourceManagerImpl<'r> {
    pub fn new(ttf_context: &'r Sdl2TtfContext) -> Self {
        ResourceManagerImpl {
            images: RefCell::new(HashMap::new()),
            fonts: RefCell::new(HashMap::new()),
            ttf_context,
            phantom_data: PhantomData,
        }
    }

    fn image_normal(&self, name: &'static str, data: &[u8]) -> Ref<Surface> {
        let key = name.to_owned();
        if self.images.borrow().get(&key).is_none() {
            self.images.borrow_mut().insert(key.clone(), load_image(data).unwrap());
        }
        Ref::map(self.images.borrow(), |r| r.get(&key).unwrap())
    }

    fn highlight(&self, name: &'static str, data: &[u8]) {
        let key = format!("{}_HIGHLIGHTED", name);
        let h = {
            let img = self.image_normal(name, data);
            adjust_brightness(&*img, 1.5)
        };
        self.images.borrow_mut().insert(key.clone(), h);
    }

    fn image_highlighted(&self, name: &'static str, data: &[u8]) -> Ref<Surface> {
        let key = format!("{}_HIGHLIGHTED", name);
        if self.images.borrow().get(&key).is_none() {
            self.highlight(name, data);
        }
        Ref::map(self.images.borrow(), |r| r.get(&key).unwrap())
    }
}

impl<'r> ResourceManager for ResourceManagerImpl<'r> {
    fn image(&self, name: &'static str, data: &[u8], highlighted: bool) -> Ref<Surface> {
        if highlighted {
            self.image_highlighted(name, data)
        } else {
            self.image_normal(name, data)
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
}
