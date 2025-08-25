use raylib::prelude::*;
use std::collections::HashMap;
use std::slice;

pub struct TextureManager {
    images:   HashMap<char, Image>,
    textures: HashMap<char, Texture2D>,
}

impl TextureManager {
    pub fn new(rl: &mut RaylibHandle, th: &RaylibThread) -> Self {

        let mapping = [

            ('1', "assets/textures/handpaintedwall2.png"),  
            (' ', "assets/textures/piso.jpeg"),     
        ];

        let mut images = HashMap::new();
        let mut textures = HashMap::new();

        for (ch, path) in mapping {
            let img = match Image::load_image(path) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("No se pudo abrir {path}: {e}.");
                    Image::gen_image_color(128, 128, Color::WHITE)
                }
            };

            if let Ok(tex) = rl.load_texture_from_image(th, &img) {
                textures.insert(ch, tex);  // Guardamos la textura en el HashMap
            }
            images.insert(ch, img);
        }

        Self { images, textures }
    }

    pub fn has_image(&self, ch: char) -> bool { self.images.contains_key(&ch) }

    pub fn image_size(&self, ch: char) -> Option<(u32, u32)> {
        self.images.get(&ch).map(|i| (i.width as u32, i.height as u32))
    }


    pub fn get_pixel_color(&self, ch: char, tx: u32, ty: u32) -> Option<Color> {
        let img = self.images.get(&ch)?;
        Some(sample_pixel(img, tx, ty))
    }

    pub fn get_texture(&self, ch: char) -> Option<&Texture2D> {
        self.textures.get(&ch)
    }
}


fn sample_pixel(image: &Image, tx: u32, ty: u32) -> Color {
    let w = image.width as usize;
    let h = image.height as usize;
    if w == 0 || h == 0 { return Color::WHITE; }

    let x = tx.min(w.saturating_sub(1) as u32) as usize;
    let y = ty.min(h.saturating_sub(1) as u32) as usize;

    let len = w * h * 4;
    unsafe {
        let data = slice::from_raw_parts(image.data as *const u8, len);
        let idx = (y * w + x) * 4;
        if idx + 3 >= len { return Color::WHITE; }
        Color::new(data[idx], data[idx + 1], data[idx + 2], data[idx + 3])
    }
}
