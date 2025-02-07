use super::U;
use rand::distr::StandardUniform;
use rand::prelude::{Distribution, Rng};
use rand::Fill;

impl<const N: usize> Fill for U<N> {
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.fill(rng)
    }
}

impl<const N: usize> Distribution<U<N>> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> U<N> {
        U(StandardUniform::sample(self, rng))
    }
}
