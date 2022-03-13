mod model;
mod utils;

extern crate oxigen;
extern crate rand;

use crate::model::element::{with_report, Element};
use clap::Parser;
use oxigen::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::fmt::Display;
use std::fs::File;

#[derive(Parser, Debug)]
#[clap(about, long_about = None)]
pub struct CLIArgs {
    #[clap(short, long, default_value_t = 100)]
    population: usize,

    #[clap(short, long, default_value_t = 100)]
    size: usize,

    #[clap(short, long, default_value_t = 0.1)]
    mut_rate: f32,

    #[clap(short, long, default_value_t = 0.5)]
    crossover_frac: f32,
}

fn main() {
    let args = CLIArgs::parse();

}
