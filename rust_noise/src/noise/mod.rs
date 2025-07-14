pub mod value_noise;
pub mod perlin_noise;
pub mod simplex_noise;

use std::default::Default;

#[derive(Clone, Copy, Debug, PartialEq)]
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
    perm: [i32; 512],
    grad3: [[f64; 3]; 12],
}

impl Default for Noise {
    fn default() -> Self {
        let p = [
            151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 223, 7, 225, 140, 36, 103, 30,
            69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62,
            94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125,
            136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
            111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143,
            54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196,
            135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250,
            124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58,
            17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221,
            153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224,
            232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12,
            191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199,
            106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
            222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
        ];

        let mut perm = [0; 512];
        for i in 0..512 {
            perm[i] = p[i & 255];
        }

        Self {
            noise_type: NoiseType::default(),
            octaves: 4,
            frequency: 0.2,
            lacunarity: 1.9,
            persistence: 1.8,
            gain: 0.33,
            perm,
            grad3: [
                [1.0, 1.0, 0.0],
                [-1.0, 1.0, 0.0],
                [1.0, -1.0, 0.0],
                [-1.0, -1.0, 0.0],
                [1.0, 0.0, 1.0],
                [-1.0, 0.0, 1.0],
                [1.0, 0.0, -1.0],
                [-1.0, 0.0, -1.0],
                [0.0, 1.0, 1.0],
                [0.0, -1.0, 1.0],
                [0.0, 1.0, -1.0],
                [0.0, -1.0, -1.0],
            ],
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
        let mut noise = Self::default();
        noise.noise_type = noise_type;
        noise.octaves = octaves;
        noise.frequency = frequency;
        noise.lacunarity = lacunarity;
        noise.persistence = persistence;
        noise.gain = gain;
        noise
    }

    fn fade(&self, t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn grad_dot(&self, hash: i32, x: f64, y: f64) -> f64 {
        let grad = self.grad3[hash as usize];
        grad[0] * x + grad[1] * y
    }

    fn grad_dot_3d(&self, hash: i32, x: f64, y: f64, z: f64) -> f64 {
        let grad = self.grad3[hash as usize];
        grad[0] * x + grad[1] * y + grad[2] * z
    }
}
