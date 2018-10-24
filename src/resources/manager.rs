use std::collections::HashMap;
use std::marker::PhantomData;
use sdl::video::Surface;
use ui::utils::{load_image, adjust_brightness};

pub struct ResourceManager<'r> {
    images: HashMap<String, Surface>,
    phantom_data: PhantomData<&'r str>,
}

impl<'r> ResourceManager<'r> {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
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
}
