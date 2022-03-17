mod model;
mod passes;
mod utils;

extern crate oxigen;
extern crate rand;

use std::fs::File;
use clap::Parser;
use oxigen::prelude::*;
use crate::model::algorithm::Algorithm;
use crate::model::pass::Pass;

#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
pub struct CLIArgs {
    #[clap(short, long, default_value_t = 100)]
    population_size: usize,

    #[clap(short, long, default_value_t = 100)]
    data_size: u32,

    #[clap(short, long, default_value_t = 0.1)]
    mut_rate: f32,

    #[clap(short, long, default_value_t = 0.5)]
    crossover_frac: f32,
}

fn main() {
    let args = CLIArgs::parse();

    let progress_log = File::create("progress.csv").expect("Error creating progress log file");
    let population_log = File::create("population.txt").expect("Error creating population log file");

    let (solutions, generation, progress, _population) = GeneticExecution::<Pass, Algorithm>::new()
        .population_size(args.population_size)
        .genotype_size(args.data_size)
        // .mutation_rate(Box::new(MutationRates::Linear(SlopeParams {
        //     start: f64::from(n_queens) / (2_f64 + 4_f64 * log2) / 100_f64,
        //     bound: 0.005,
        //     coefficient: -0.00002,
        // })))
        // .selection_rate(Box::new(SelectionRates::Linear(SlopeParams {
        //     start: 3_f64,
        //     bound: 6_f64,
        //     coefficient: 0.05,
        // })))
        .select_function(Box::new(SelectionFunctions::Tournaments(NTournaments(
            args.population_size / 2,
        ))))
        .crossover_function(Box::new(CrossoverFunctions::UniformCross))
        // .population_refitness_function(Box::new(PopulationRefitnessFunctions::Niches(
        //     NichesAlpha(0.8),
        //     Box::new(NichesBetaRates::Linear(SlopeParams {
        //         start: 0.0025,
        //         bound: 10.0_f64.min(log2 * log2 / 6.0),
        //         coefficient: 0.000001 * log2 * log2,
        //     })),
        //     NichesSigma(0.6),
        // )))
        // .survival_pressure_function(Box::new(
        //     SurvivalPressureFunctions::ChildrenFightParentsAndTheRestWorst,
        // ))
        // .survival_pressure_function(Box::new(SurvivalPressureFunctions::Worst))
        .age_function(Box::new(AgeFunctions::Linear(
            AgeThreshold(5),
            AgeSlope(0.5),
        )))
        .stop_criterion(Box::new(StopCriteria::Generation(200)))
        .progress_log(20, progress_log)
        .population_log(20, population_log)
        .run();

    println!(
        "Finished in the generation {} with a progress of {}",
        generation, progress
    );
    for sol in &solutions {
        println!("{}", sol);
    }
}
