mod math;
mod noise;

fn main() {
    let width = 512;
    let height = 512;
    let frequency = 8.0;
    let amplitude = 1.0;

    let noise = noise::perlin::PerlinNoise::new();

    let mut img = image::GrayImage::new(width, height);
    img.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let x = (x as f32 / width as f32) * frequency;
        let y = (y as f32 / height as f32) * frequency;
        let noise_val = noise.calculate_noise(math::vec::Vec2::new(x, y)) * amplitude;

        let clamped = ((noise_val + 1.0) * 127.5).clamp(0.0, 255.0) as u8;
        *pixel = image::Luma([clamped]);
    });

    img.save("perlin_noise.png").unwrap();
}
