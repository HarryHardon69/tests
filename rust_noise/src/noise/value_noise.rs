use crate::noise::Noise;

impl Noise {
    pub fn value(&self, x: f64, y: f64) -> f64 {
        let i = x.floor() as i32;
        let j = y.floor() as i32;

        let ii = i & 255;
        let jj = j & 255;

        let nll = self.perm[ii as usize + self.perm[jj as usize] as usize] as f64 / 255.0;
        let nhl = self.perm[ii as usize + self.perm[(jj + 1) as usize] as usize] as f64 / 255.0;
        let nlh = self.perm[(ii + 1) as usize + self.perm[jj as usize] as usize] as f64 / 255.0;
        let nhh = self.perm[(ii + 1) as usize + self.perm[(jj + 1) as usize] as usize] as f64 / 255.0;

        let u = x - i as f64;
        let v = y - j as f64;

        let nyl = nll + ((nhl - nll) * u);
        let nyh = nlh + ((nhh - nlh) * u);

        let nxy = nyl + ((nyh - nyl) * v);

        nxy * 2.0 - 1.0
    }

    pub fn value_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let i = x.floor() as i32;
        let j = y.floor() as i32;
        let k = z.floor() as i32;

        let ii = i & 255;
        let jj = j & 255;
        let kk = k & 255;

        let nlll = self.perm[ii as usize + self.perm[jj as usize + self.perm[kk as usize] as usize] as usize] as f64 / 255.0;
        let nlhl = self.perm[ii as usize + self.perm[(jj + 1) as usize + self.perm[kk as usize] as usize] as usize] as f64 / 255.0;
        let nhll = self.perm[(ii + 1) as usize + self.perm[jj as usize + self.perm[kk as usize] as usize] as usize] as f64 / 255.0;
        let nhhl = self.perm[(ii + 1) as usize + self.perm[(jj + 1) as usize + self.perm[kk as usize] as usize] as usize] as f64 / 255.0;
        let nllh = self.perm[ii as usize + self.perm[jj as usize + self.perm[(kk + 1) as usize] as usize] as usize] as f64 / 255.0;
        let nlhh = self.perm[ii as usize + self.perm[(jj + 1) as usize + self.perm[(kk + 1) as usize] as usize] as usize] as f64 / 255.0;
        let nhlh = self.perm[(ii + 1) as usize + self.perm[jj as usize + self.perm[(kk + 1) as usize] as usize] as usize] as f64 / 255.0;
        let nhhh = self.perm[(ii + 1) as usize + self.perm[(jj + 1) as usize + self.perm[(kk + 1) as usize] as usize] as usize] as f64 / 255.0;

        let u = x - i as f64;
        let v = y - j as f64;
        let w = z - k as f64;

        let nyll = nlll + ((nhll - nlll) * u);
        let nyhl = nlhl + ((nhhl - nlhl) * u);
        let nylh = nllh + ((nhlh - nllh) * u);
        let nyhh = nlhh + ((nhhh - nlhh) * u);

        let nxyl = nyll + ((nyhl - nyll) * v);
        let nxyh = nylh + ((nyhh - nylh) * v);

        let nxyz = nxyl + ((nxyh - nxyl) * w);

        nxyz * 2.0 - 1.0
    }
}
