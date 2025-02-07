use super::I;
use rand::distr::StandardUniform;
use rand::prelude::{Distribution, Rng};
use rand::Fill;

impl<const N: usize> Fill for I<N> {
    fn fill<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.0.fill(rng)
    }
}

impl<const N: usize> Distribution<I<N>> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> I<N> {
        I(StandardUniform::sample(self, rng))
    }
}
