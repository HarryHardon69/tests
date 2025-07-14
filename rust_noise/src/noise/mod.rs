pub mod value_noise;
pub mod perlin_noise;
pub mod simplex_noise;

use std::default::Default;

#[derive(Clone, Copy, Debug)]
pub enum NoiseType {
    Value,
    Perlin,
    Simplex,
}

impl Default for NoiseType {
    fn default() -> Self {
        NoiseType::Simplex
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Noise {
    pub noise_type: NoiseType,
    pub octaves: i32,
    pub frequency: f64,
    pub lacunarity: f64,
    pub persistence: f64,
    pub gain: f64,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            noise_type: NoiseType::default(),
            octaves: 4,
            frequency: 0.2,
            lacunarity: 1.9,
            persistence: 1.8,
            gain: 0.33,
        }
    }
}

impl Noise {
    pub fn new(
        noise_type: NoiseType,
        octaves: i32,
        frequency: f64,
        lacunarity: f64,
        persistence: f64,
        gain: f64,
    ) -> Self {
        Self {
            noise_type,
            octaves,
            frequency,
            lacunarity,
            persistence,
            gain,
        }
    }
}
