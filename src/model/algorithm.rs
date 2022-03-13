use super::pass::Pass;
use std::fmt::{Display, Formatter};
use std::slice::Iter;
use std::vec::IntoIter;

use crate::model::pass::get_random_pass_type;
use crate::utils::gaps::generate_geometric_gaps;
use crate::utils::slice::rand_index;
use oxigen::prelude::Genotype;
use rand::distributions::Uniform;
use rand::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, std::hash::Hash)]
pub struct Algorithm {
    passes: Vec<Pass>,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Genotype<Pass> for Algorithm {
    type ProblemSize = u32;
    type GenotypeHash = Self;

    fn iter(&self) -> Iter<Pass> {
        self.passes.iter()
    }

    fn into_iter(self) -> IntoIter<Pass> {
        self.passes.into_iter()
    }

    fn from_iter<I: Iterator<Item = Pass>>(&mut self, genes: I) {
        self.passes = genes.collect();
    }

    fn generate(size: &Self::ProblemSize) -> Self {
        let mut rgen = SmallRng::from_entropy();
        let gaps = generate_geometric_gaps(*size, rgen.sample(Uniform::from(1.25..2.5)));
        let mut passes = Vec::with_capacity(*size as usize);

        gaps.iter().for_each(|gap| {
            passes.push(Pass {
                pass_type: get_random_pass_type(&mut rgen),
                gap: *gap,
            })
        });

        Algorithm { passes }
    }

    fn fitness(&self) -> f64 {
        todo!()
    }

    fn mutate(&mut self, rgen: &mut SmallRng, index: usize) {
        self.passes[index].mutate(rgen);
    }

    fn is_solution(&self, _fitness: f64) -> bool {
        // TODO: any way to determine correctly?
        false
    }

    fn hash(&self) -> Self::GenotypeHash {
        self.clone()
    }
}
