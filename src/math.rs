mod common {
    use num_traits::{Float, Num, NumCast};

    pub trait Number: Num + NumCast + Copy {}
    impl<T: Num + NumCast + Copy> Number for T {}

    pub trait FloatNumber: Float + Copy {}
    impl<T: Float + Copy> FloatNumber for T {}
}

pub mod vec {
    use super::common::{FloatNumber, Number};

    pub trait VectorConstants {
        const ZERO: Self;
        const ONE: Self;
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Vec2<T> {
        pub x: T,
        pub y: T,
    }

    impl<T> Vec2<T>
    where
        T: Number,
    {
        pub fn new(x: T, y: T) -> Self {
            Vec2 { x, y }
        }
    }

    impl<T> Vec2<T>
    where
        T: FloatNumber,
    {
        pub fn dot(&self, other: &Self) -> T {
            self.x * other.x + self.y * other.y
        }

        pub fn magnitude(&self) -> T {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }

    impl Vec2<f32> {
        pub fn random_unit_vec() -> Vec2<f32> {
            let angle = fastrand::f32() * std::f32::consts::PI * 2.0;
            Vec2 {
                x: angle.cos(),
                y: angle.sin(),
            }
        }
    }

    impl<T> std::ops::Add for Vec2<T>
    where
        T: Number,
    {
        type Output = Vec2<T>;

        fn add(self, other: Vec2<T>) -> Vec2<T> {
            Vec2 {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl<T> std::ops::Sub for Vec2<T>
    where
        T: Number,
    {
        type Output = Vec2<T>;

        fn sub(self, other: Vec2<T>) -> Vec2<T> {
            Vec2 {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl<T> std::ops::Mul<T> for Vec2<T>
    where
        T: Number,
    {
        type Output = Vec2<T>;

        fn mul(self, scalar: T) -> Vec2<T> {
            Vec2 {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    impl<T> std::ops::Div<T> for Vec2<T>
    where
        T: Number,
    {
        type Output = Vec2<T>;

        fn div(self, scalar: T) -> Vec2<T> {
            Vec2 {
                x: self.x / scalar,
                y: self.y / scalar,
            }
        }
    }

    impl VectorConstants for Vec2<f32> {
        const ZERO: Self = Vec2 { x: 0.0, y: 0.0 };
        const ONE: Self = Vec2 { x: 1.0, y: 1.0 };
    }

    impl VectorConstants for Vec2<i32> {
        const ZERO: Self = Vec2 { x: 0, y: 0 };
        const ONE: Self = Vec2 { x: 1, y: 1 };
    }

    impl VectorConstants for Vec2<u32> {
        const ZERO: Self = Vec2 { x: 0, y: 0 };
        const ONE: Self = Vec2 { x: 1, y: 1 };
    }

    impl VectorConstants for Vec2<usize> {
        const ZERO: Self = Vec2 { x: 0, y: 0 };
        const ONE: Self = Vec2 { x: 1, y: 1 };
    }
}

pub mod interp {
    pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + (b - a) * t
    }

    pub fn fade(t: f32) -> f32 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }
}
