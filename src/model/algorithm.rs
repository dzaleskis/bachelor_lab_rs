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
use crate::model::element::with_report;
use crate::utils::test_data::get_test_data;
use crate::passes::runner::run_pass;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Algorithm {
    passes: Vec<Pass>,
    data_size: u32,
}

impl Display for Algorithm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Genotype<Pass> for Algorithm {
    type ProblemSize = u32;

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

        Algorithm { passes, data_size: *size }
    }

    fn fitness(&self) -> f64 {
        // FITNESS, NOT COST (HIGHER IS BETTER)
        let mut data = get_test_data(self.data_size as usize);
        let min_comparisons = 12000.0;
        let min_assignments = 13000.0;

        let (report, _) = with_report(|| self.passes.iter().rev().for_each(|pass| run_pass(pass, &mut data)));
        let comparisons = report.comparisons as f64;
        let assignments = report.assignments as f64;

        let normalized_comparisons = min_comparisons / comparisons;
        let normalized_assignments = min_assignments / assignments;

        return (0.5 * normalized_comparisons) + (0.5 * normalized_assignments);
    }

    fn mutate(&mut self, rgen: &mut SmallRng, index: usize) {
        self.passes[index].mutate(rgen);
    }

    fn is_solution(&self, _fitness: f64) -> bool {
        // TODO: any way to determine correctly?
        false
    }
}
