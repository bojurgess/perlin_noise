pub mod perlin {
    use crate::math::{
        interp::{fade, lerp},
        vec::Vec2,
    };

    pub struct PerlinNoise {
        gradients: Vec<Vec2<f32>>,
        permutations: Vec<usize>,
    }

    impl PerlinNoise {
        pub fn new() -> Self {
            PerlinNoise {
                gradients: Self::gen_gradients(),
                permutations: Self::gen_permutations(),
            }
        }

        pub fn calculate_noise(&self, p: Vec2<f32>) -> f32 {
            let mut offsets = Vec::new();
            let mut dots = Vec::new();

            for &corner in &[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0), (1.0, 1.0)] {
                // always start in bottom left corner and add an offset to get the other corners
                let offset = Vec2::new(p.x.floor() + corner.0, p.y.floor() + corner.1);
                let dv = p - offset;

                let i = self.permutations
                    [(offset.x as usize + self.permutations[offset.y as usize % 256]) % 256];
                let gv = self.gradients[i % 8];

                offsets.push(offset);
                dots.push(gv.dot(&dv));
            }

            // interpolation
            let u = fade(p.x - offsets[0].x);
            let v = fade(p.y - offsets[0].y);

            let interp_x0 = lerp(dots[0], dots[1], u);
            let interp_x1 = lerp(dots[2], dots[3], u);

            lerp(interp_x0, interp_x1, v)
        }

        fn gen_gradients() -> Vec<Vec2<f32>> {
            let mut gradients = Vec::new();
            for _ in 0..8 {
                gradients.push(Vec2::random_unit_vec());
            }
            gradients
        }

        fn gen_permutations() -> Vec<usize> {
            let mut permutations = (0..256).collect::<Vec<usize>>();
            fastrand::shuffle(&mut permutations);
            permutations
        }
    }
}
