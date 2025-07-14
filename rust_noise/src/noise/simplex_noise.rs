use crate::noise::Noise;

const F2: f64 = 0.3660254037844386;
const G2: f64 = 0.21132486540518713;

impl Noise {
    pub fn simplex(&self, x: f64, y: f64) -> f64 {
        let s = (x + y) * F2;
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;

        let t = (i + j) as f64 * G2;

        let x0 = x - (i as f64 - t);
        let y0 = y - (j as f64 - t);

        let (i1, j1) = if x0 > y0 { (1, 0) } else { (0, 1) };

        let x1 = x0 - i1 as f64 + G2;
        let y1 = y0 - j1 as f64 + G2;
        let x2 = x0 - 1.0 + 2.0 * G2;
        let y2 = y0 - 1.0 + 2.0 * G2;

        let ii = i & 255;
        let jj = j & 255;

        let gi0 = self.perm[ii as usize + self.perm[jj as usize] as usize] % 12;
        let gi1 = self.perm[(ii + i1) as usize + self.perm[(jj + j1) as usize] as usize] % 12;
        let gi2 = self.perm[(ii + 1) as usize + self.perm[(jj + 1) as usize] as usize] % 12;

        let t0 = 0.5 - x0 * x0 - y0 * y0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            let t0 = t0 * t0;
            t0 * t0 * self.grad_dot(gi0, x0, y0)
        };

        let t1 = 0.5 - x1 * x1 - y1 * y1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            let t1 = t1 * t1;
            t1 * t1 * self.grad_dot(gi1, x1, y1)
        };

        let t2 = 0.5 - x2 * x2 - y2 * y2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            let t2 = t2 * t2;
            t2 * t2 * self.grad_dot(gi2, x2, y2)
        };

        70.0 * (n0 + n1 + n2)
    }

    pub fn simplex_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let s = (x + y + z) * (1.0 / 3.0);
        let i = (x + s).floor() as i32;
        let j = (y + s).floor() as i32;
        let k = (z + s).floor() as i32;

        let t = (i + j + k) as f64 * (1.0 / 6.0);

        let x0 = x - (i as f64 - t);
        let y0 = y - (j as f64 - t);
        let z0 = z - (k as f64 - t);

        let (i1, j1, k1, i2, j2, k2) = if x0 >= y0 {
            if y0 >= z0 {
                (1, 0, 0, 1, 1, 0)
            } else if x0 >= z0 {
                (1, 0, 0, 1, 0, 1)
            } else {
                (0, 0, 1, 1, 0, 1)
            }
        } else {
            if y0 < z0 {
                (0, 0, 1, 0, 1, 1)
            } else if x0 < z0 {
                (0, 1, 0, 0, 1, 1)
            } else {
                (0, 1, 0, 1, 1, 0)
            }
        };

        let x1 = x0 - i1 as f64 + (1.0 / 6.0);
        let y1 = y0 - j1 as f64 + (1.0 / 6.0);
        let z1 = z0 - k1 as f64 + (1.0 / 6.0);
        let x2 = x0 - i2 as f64 + (2.0 / 6.0);
        let y2 = y0 - j2 as f64 + (2.0 / 6.0);
        let z2 = z0 - k2 as f64 + (2.0 / 6.0);
        let x3 = x0 - 1.0 + (3.0 / 6.0);
        let y3 = y0 - 1.0 + (3.0 / 6.0);
        let z3 = z0 - 1.0 + (3.0 / 6.0);

        let ii = i & 255;
        let jj = j & 255;
        let kk = k & 255;

        let gi0 = self.perm[ii as usize + self.perm[jj as usize + self.perm[kk as usize] as usize] as usize] % 12;
        let gi1 = self.perm[(ii + i1) as usize + self.perm[(jj + j1) as usize + self.perm[(kk + k1) as usize] as usize] as usize] % 12;
        let gi2 = self.perm[(ii + i2) as usize + self.perm[(jj + j2) as usize + self.perm[(kk + k2) as usize] as usize] as usize] % 12;
        let gi3 = self.perm[(ii + 1) as usize + self.perm[(jj + 1) as usize + self.perm[(kk + 1) as usize] as usize] as usize] % 12;

        let t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
        let n0 = if t0 < 0.0 {
            0.0
        } else {
            let t0 = t0 * t0;
            t0 * t0 * self.grad_dot_3d(gi0, x0, y0, z0)
        };

        let t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
        let n1 = if t1 < 0.0 {
            0.0
        } else {
            let t1 = t1 * t1;
            t1 * t1 * self.grad_dot_3d(gi1, x1, y1, z1)
        };

        let t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
        let n2 = if t2 < 0.0 {
            0.0
        } else {
            let t2 = t2 * t2;
            t2 * t2 * self.grad_dot_3d(gi2, x2, y2, z2)
        };

        let t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
        let n3 = if t3 < 0.0 {
            0.0
        } else {
            let t3 = t3 * t3;
            t3 * t3 * self.grad_dot_3d(gi3, x3, y3, z3)
        };

        32.0 * (n0 + n1 + n2 + n3)
    }
}
