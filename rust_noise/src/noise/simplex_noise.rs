use noise::{NoiseFn, Point2, Point3, Simplex};

pub fn simplex_noise_2d(point: Point2<f64>, seed: u32) -> f64 {
    Simplex::new(seed).get(point)
}

pub fn simplex_noise_3d(point: Point3<f64>, seed: u32) -> f64 {
    Simplex::new(seed).get(point)
}
