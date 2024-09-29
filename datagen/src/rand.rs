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
    const N: f32 = 1_000_000.0;
    const C: f32 = 82.528;
    const PI: f32 = 0.5;
    const EXPECTED: f32 = N * PI;

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

        let sum: f32 = results
            .iter()
            .map(|observed| (*observed as f32 - EXPECTED).powf(2.0) / EXPECTED)
            .sum();

        assert!(
            sum < C,
            "Failed chi-square for 64 degrees of freedom at a 5% significance level with a value of {}",
            sum
        );
    }
}
