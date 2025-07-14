use noise::{NoiseFn, Point2, Point3, Value};

pub fn value_noise_2d(point: Point2<f64>, seed: u32) -> f64 {
    Value::new(seed).get(point)
}

pub fn value_noise_3d(point: Point3<f64>, seed: u32) -> f64 {
    Value::new(seed).get(point)
}
