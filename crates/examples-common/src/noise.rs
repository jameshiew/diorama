//! Minimal 2D Perlin noise + fractal Brownian motion for example scenes.
//!
//! Replaces the `noise` crate for texture/terrain generation in the examples.
//! Output of `Perlin::get` is approximately in `[-1, 1]`.

pub struct Perlin {
    perm: [u8; 512],
}

impl Perlin {
    pub fn new(seed: u32) -> Self {
        let mut p: [u8; 256] = core::array::from_fn(|i| i as u8);

        let mut state = u64::from(seed).wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut next = || -> u64 {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };

        for i in (1..256).rev() {
            let j = (next() % (i as u64 + 1)) as usize;
            p.swap(i, j);
        }

        let mut perm = [0u8; 512];
        perm[..256].copy_from_slice(&p);
        perm[256..].copy_from_slice(&p);
        Self { perm }
    }

    pub fn get(&self, point: [f64; 2]) -> f64 {
        let [x, y] = point;
        let xf = x.floor();
        let yf = y.floor();
        let xi = (xf as i64 & 255) as usize;
        let yi = (yf as i64 & 255) as usize;
        let x = x - xf;
        let y = y - yf;

        let u = fade(x);
        let v = fade(y);

        let p = &self.perm;
        let a = p[xi] as usize + yi;
        let b = p[xi + 1] as usize + yi;

        let x1 = lerp(grad(p[a], x, y), grad(p[b], x - 1.0, y), u);
        let x2 = lerp(
            grad(p[a + 1], x, y - 1.0),
            grad(p[b + 1], x - 1.0, y - 1.0),
            u,
        );
        lerp(x1, x2, v)
    }
}

pub struct Fbm {
    perlin: Perlin,
    octaves: u32,
    frequency: f64,
    lacunarity: f64,
    persistence: f64,
}

impl Fbm {
    pub fn new(seed: u32) -> Self {
        Self {
            perlin: Perlin::new(seed),
            octaves: 6,
            frequency: 1.0,
            lacunarity: 2.0,
            persistence: 0.5,
        }
    }

    pub fn set_octaves(mut self, octaves: u32) -> Self {
        self.octaves = octaves;
        self
    }

    pub fn set_frequency(mut self, frequency: f64) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn get(&self, point: [f64; 2]) -> f64 {
        let [x, y] = point;
        let mut total = 0.0;
        let mut frequency = self.frequency;
        let mut amplitude = 1.0;
        for _ in 0..self.octaves {
            total += self.perlin.get([x * frequency, y * frequency]) * amplitude;
            amplitude *= self.persistence;
            frequency *= self.lacunarity;
        }
        total
    }
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

fn grad(hash: u8, x: f64, y: f64) -> f64 {
    match hash & 7 {
        0 => x + y,
        1 => -x + y,
        2 => x - y,
        3 => -x - y,
        4 => x,
        5 => -x,
        6 => y,
        _ => -y,
    }
}
