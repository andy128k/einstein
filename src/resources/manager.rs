use std::collections::HashMap;
use std::marker::PhantomData;
use sdl::video::Surface;
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};
use ui::utils::{load_image, adjust_brightness};

const FONT_DUMP: &[u8] = include_bytes!("./fonts/LiberationSans-Regular.ttf"); // /usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf

pub struct ResourceManager<'r> {
    images: HashMap<String, Surface>,
    fonts: HashMap<u16, Font<'r, 'r>>,
    ttf_context: &'r Sdl2TtfContext,
    phantom_data: PhantomData<&'r str>,
}

impl<'r> ResourceManager<'r> {
    pub fn new(ttf_context: &'r Sdl2TtfContext) -> Self {
        Self {
            images: HashMap::new(),
            fonts: HashMap::new(),
            ttf_context,
            phantom_data: PhantomData,
        }
    }

    pub fn image(&mut self, name: &'static str, data: &[u8]) -> &Surface {
        self.images.entry(name.to_owned()).or_insert_with(|| load_image(data).unwrap())
    }

    fn highlight(&mut self, name: &'static str, data: &[u8]) {
        let key = format!("{}_HIGHLIGHTED", name);
        let h = {
            let img = self.image(name, data);
            adjust_brightness(img, 1.5)
        };
        self.images.insert(key.clone(), h);
    }

    pub fn image_highlighted(&mut self, name: &'static str, data: &[u8]) -> &Surface {
        let key = format!("{}_HIGHLIGHTED", name);
        if self.images.get(&key).is_none() {
            self.highlight(name, data);
        }
        self.images.get(&key).unwrap()
    }

    pub fn image_h(&mut self, name: &'static str, data: &[u8], highlighted: bool) -> &Surface {
        if highlighted {
            self.image_highlighted(name, data)
        } else {
            self.image(name, data)
        }
    }

    pub fn font(&mut self, point_size: u16) -> &Font<'r, 'r> {
        if self.fonts.get(&point_size).is_none() {
            let ops = RWops::from_bytes(FONT_DUMP).unwrap();
            let font = self.ttf_context.load_font_from_rwops(ops, point_size).unwrap();
            self.fonts.insert(point_size, font);
        }
        self.fonts.get(&point_size).unwrap()
    }
}
