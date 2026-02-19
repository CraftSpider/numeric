use super::I;
use rand::distr::StandardUniform;
use rand::prelude::{Distribution, Rng};
use rand::Fill;

impl<const N: usize> Fill for I<N> {
    fn fill_slice<R: Rng + ?Sized>(val: &mut [Self], rng: &mut R) {
        for v in val {
            u8::fill_slice(&mut v.0, rng)
        }
    }
}

impl<const N: usize> Distribution<I<N>> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> I<N> {
        I(StandardUniform::sample(self, rng))
    }
}
