use super::I;
use rand::distributions::Standard;
use rand::prelude::{Distribution, Rng};
use rand::{Error, Fill};

impl<const N: usize> Fill for I<N> {
    fn try_fill<R: Rng + ?Sized>(&mut self, rng: &mut R) -> Result<(), Error> {
        self.0.try_fill(rng)
    }
}

impl<const N: usize> Distribution<I<N>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> I<N> {
        I(Standard::sample(self, rng))
    }
}
