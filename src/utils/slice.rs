use rand::distributions::Uniform;
use rand::prelude::SmallRng;
use rand::Rng;

#[inline]
pub fn rand_index<T>(slice: &[T], rgen: &mut SmallRng) -> usize {
    rgen.sample(Uniform::from(0..slice.len()))
}
