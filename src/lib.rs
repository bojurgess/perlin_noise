use wasm_bindgen::prelude::*;

mod math;
mod noise;
mod util;

#[wasm_bindgen]
pub struct PerlinNoise {
    noise: noise::perlin::PerlinNoise,
}

#[wasm_bindgen]
impl PerlinNoise {
    pub fn new() -> Self {
        util::set_panic_hook();
        Self {
            noise: noise::perlin::PerlinNoise::new(),
        }
    }

    pub fn generate_image(
        &self,
        width: u32,
        height: u32,
        frequency: f32,
        amplitude: f32,
    ) -> Vec<u8> {
        let mut img = image::GrayImage::new(width, height);
        img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let x = (x as f32 / width as f32) * frequency;
            let y = (y as f32 / height as f32) * frequency;
            let noise_val = self.noise.calculate_noise(math::vec::Vec2::new(x, y)) * amplitude;

            let clamped = ((noise_val + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
            *pixel = image::Luma([clamped]);
        });

        let mut buf = Vec::new();
        use std::io::Cursor;
        let mut cursor = Cursor::new(&mut buf);
        img.write_to(&mut cursor, image::ImageFormat::Png).unwrap();

        buf
    }
}
