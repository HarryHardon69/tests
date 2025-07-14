use crate::noise::Noise;

impl Noise {
    pub fn perlin(&self, x: f64, y: f64) -> f64 {
        let i = x.floor() as i32;
        let j = y.floor() as i32;

        let x = x - i as f64;
        let y = y - j as f64;

        let i = i & 255;
        let j = j & 255;

        let gll = self.perm[i as usize + self.perm[j as usize] as usize] % 12;
        let glh = self.perm[i as usize + self.perm[(j + 1) as usize] as usize] % 12;
        let ghl = self.perm[(i + 1) as usize + self.perm[j as usize] as usize] % 12;
        let ghh = self.perm[(i + 1) as usize + self.perm[(j + 1) as usize] as usize] % 12;

        let nll = self.grad_dot(gll, x, y);
        let nlh = self.grad_dot(glh, x, y - 1.0);
        let nhl = self.grad_dot(ghl, x - 1.0, y);
        let nhh = self.grad_dot(ghh, x - 1.0, y - 1.0);

        let u = self.fade(x);
        let v = self.fade(y);

        let nyl = nll + ((nhl - nll) * u);
        let nyh = nlh + ((nhh - nlh) * u);

        let nxy = nyl + ((nyh - nyl) * v);

        nxy
    }

    pub fn perlin_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let i = x.floor() as i32;
        let j = y.floor() as i32;
        let k = z.floor() as i32;

        let x = x - i as f64;
        let y = y - j as f64;
        let z = z - k as f64;

        let i = i & 255;
        let j = j & 255;
        let k = k & 255;

        let glll = self.perm[i as usize + self.perm[j as usize + self.perm[k as usize] as usize] as usize] % 12;
        let glhl = self.perm[i as usize + self.perm[(j + 1) as usize + self.perm[k as usize] as usize] as usize] % 12;
        let ghll = self.perm[(i + 1) as usize + self.perm[j as usize + self.perm[k as usize] as usize] as usize] % 12;
        let ghhl = self.perm[(i + 1) as usize + self.perm[(j + 1) as usize + self.perm[k as usize] as usize] as usize] % 12;
        let gllh = self.perm[i as usize + self.perm[j as usize + self.perm[(k + 1) as usize] as usize] as usize] % 12;
        let glhh = self.perm[i as usize + self.perm[(j + 1) as usize + self.perm[(k + 1) as usize] as usize] as usize] % 12;
        let ghlh = self.perm[(i + 1) as usize + self.perm[j as usize + self.perm[(k + 1) as usize] as usize] as usize] % 12;
        let ghhh = self.perm[(i + 1) as usize + self.perm[(j + 1) as usize + self.perm[(k + 1) as usize] as usize] as usize] % 12;

        let nlll = self.grad_dot_3d(glll, x, y, z);
        let nlhl = self.grad_dot_3d(glhl, x, y - 1.0, z);
        let nhll = self.grad_dot_3d(ghll, x - 1.0, y, z);
        let nhhl = self.grad_dot_3d(ghhl, x - 1.0, y - 1.0, z);
        let nllh = self.grad_dot_3d(gllh, x, y, z - 1.0);
        let nlhh = self.grad_dot_3d(glhh, x, y - 1.0, z - 1.0);
        let nhlh = self.grad_dot_3d(ghlh, x - 1.0, y, z - 1.0);
        let nhhh = self.grad_dot_3d(ghhh, x - 1.0, y - 1.0, z - 1.0);

        let u = self.fade(x);
        let v = self.fade(y);
        let w = self.fade(z);

        let nxll = nlll + ((nhll - nlll) * u);
        let nxlh = nllh + ((nhlh - nllh) * u);
        let nxhl = nlhl + ((nhhl - nlhl) * u);
        let nxhh = nlhh + ((nhhh - nlhh) * u);

        let nxyl = nxll + ((nxhl - nxll) * v);
        let nxyh = nxlh + ((nxhh - nxlh) * v);

        let nxyz = nxyl + ((nxyh - nxyh) * w);

        nxyz
    }
}
