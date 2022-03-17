use crate::model::element::Element;
use crate::model::pass::{Pass, PassType};
use crate::passes::traced::*;

pub fn run_pass<T: Copy + Ord>(pass: &Pass, slice: &mut [Element<T>]) {
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
