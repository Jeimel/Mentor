use std::time::{SystemTime, UNIX_EPOCH};

pub struct Rand {
    pub seed: u128,
    multiplier: u128,
}

impl Default for Rand {
    fn default() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        Self {
            seed: nanos | 1,
            multiplier: 0xF1C47040DE494ACC251D055F00F0A1AB,
        }
    }
}

#[allow(dead_code)]
impl Rand {
    pub fn random(&mut self) -> f64 {
        self.random_raw() as f64 / u64::MAX as f64
    }

    pub fn random_range(&mut self, min: usize, max: usize) -> usize {
        min + (self.random() * (max - min) as f64) as usize
    }

    pub fn random_distributed(&mut self) -> f64 {
        let x = 1.0 - self.random();
        let y = 1.0 - self.random();

        (-2.0 * x.ln()).sqrt() * (2.0 * std::f64::consts::PI * y).cos()
    }

    pub fn random_raw(&mut self) -> u64 {
        self.seed = self.seed.wrapping_mul(self.multiplier);
        let rot = (self.seed >> 122) as u32;
        let xsl = (self.seed >> 64) as u64 ^ self.seed as u64;
        xsl.rotate_right(rot)
    }
}

#[cfg(test)]
mod rand_test {
    const N: f64 = 1_000_000.0;
    const C: f64 = 3.841;

    #[test]
    fn assert_random_distribution() {
        let mut rand = crate::rand::Rand::default();
        let mut results = [0u64; 64];

        for _ in 0..N as u64 {
            let bits = rand.random_raw();

            for (i, result) in results.iter_mut().enumerate() {
                *result += (bits >> i) & 1;
            }
        }

        let pi = 0.5f64;

        for n_i in results {
            let x_square = (n_i as f64 - N * pi).powf(2.0) / N * 2.0;

            assert!(x_square < C)
        }
    }
}
