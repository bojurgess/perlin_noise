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
            let offsets = [
                Vec2::new(p.x.floor(), p.y.floor()),
                Vec2::new(p.x.ceil(), p.y.floor()),
                Vec2::new(p.x.floor(), p.y.ceil()),
                Vec2::new(p.x.ceil(), p.y.ceil()),
            ];

            let dv = [
                p - offsets[0],
                p - offsets[1],
                p - offsets[2],
                p - offsets[3],
            ];

            let gv = [
                self.gradients[self.permutations[(offsets[0].x as usize
                    + self.permutations[offsets[0].y as usize % 256])
                    % 256]
                    % 8],
                self.gradients[self.permutations[(offsets[1].x as usize
                    + self.permutations[offsets[1].y as usize % 256])
                    % 256]
                    % 8],
                self.gradients[self.permutations[(offsets[2].x as usize
                    + self.permutations[offsets[2].y as usize % 256])
                    % 256]
                    % 8],
                self.gradients[self.permutations[(offsets[3].x as usize
                    + self.permutations[offsets[3].y as usize % 256])
                    % 256]
                    % 8],
            ];

            let dots = [
                gv[0].dot(&dv[0]),
                gv[1].dot(&dv[1]),
                gv[2].dot(&dv[2]),
                gv[3].dot(&dv[3]),
            ];

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
