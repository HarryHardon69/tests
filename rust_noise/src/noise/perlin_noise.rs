use noise::{NoiseFn, Perlin, Point2, Point3};

pub fn perlin_noise_2d(point: Point2<f64>, seed: u32) -> f64 {
    Perlin::new(seed).get(point)
}

pub fn perlin_noise_3d(point: Point3<f64>, seed: u32) -> f64 {
    Perlin::new(seed).get(point)
}
