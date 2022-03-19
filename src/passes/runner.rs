use crate::model::element::{Element, Report};
use crate::model::pass::{Pass, PassType};
use crate::passes::traced::*;

pub fn run_pass<T: Copy + Ord>(pass: &Pass, slice: &mut [Element<T>]) -> Report {
    let gap = pass.gap as usize;
    let pass_type = pass.pass_type;

    match pass_type {
        PassType::Bubble => bubble_pass(slice, gap),
        PassType::Brick => brick_pass(slice, gap),
        PassType::Shake => shake_pass(slice, gap),
        PassType::Insertion => insertion_pass(slice, gap),
        PassType::InsertionImproved => insertion_improved_pass(slice, gap),
    }
}

pub fn run_passes<T: Copy + Ord>(passes: &[Pass], slice: &mut [Element<T>]) -> Report {
    // use a local variable to store the collected report from all the passes
    let mut report: Report = Default::default();

    // gaps in passes are stored from lowest to highest
    // when sorting, we need to reverse them, so sorting with the largest gap goes first
    passes
        .iter()
        .rev()
        .for_each(|pass| report.add(&run_pass(pass, slice)));

    report
}
