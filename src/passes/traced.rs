use crate::model::element::{Element, Report, Reporter};
use std::cmp::Ordering;

fn is_greater<T: Copy + Ord>(a: &Element<T>, b: &Element<T>, rep: &mut Reporter) -> bool {
    Element::compare(&a, &b, rep) == Ordering::Greater
}

fn is_lesser<T: Copy + Ord>(a: &Element<T>, b: &Element<T>, rep: &mut Reporter) -> bool {
    Element::compare(&a, &b, rep) == Ordering::Less
}

pub fn brick_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) -> Report {
    let n = slice.len() - gap;
    let step = gap * 2;
    let mut rep = Reporter::new();

    for i in (gap..n).step_by(step) {
        if is_greater(&slice[i], &slice[i + gap], &mut rep) {
            Element::swap(slice, i, i + gap, &mut rep);
        }
    }

    for i in (0..n).step_by(step) {
        if is_greater(&slice[i], &slice[i + gap], &mut rep) {
            Element::swap(slice, i, i + gap, &mut rep);
        }
    }

    rep.get_report()
}

pub fn shake_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) -> Report {
    let n = slice.len() - gap;
    let mut rep = Reporter::new();

    for i in 0..n {
        if is_greater(&slice[i], &slice[i + gap], &mut rep) {
            Element::swap(slice, i, i + gap, &mut rep);
        }
    }

    for i in (0..n).rev() {
        if is_greater(&slice[i], &slice[i + gap], &mut rep) {
            Element::swap(slice, i, i + gap, &mut rep);
        }
    }

    rep.get_report()
}

pub fn bubble_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) -> Report {
    let n = slice.len() - gap;
    let mut rep = Reporter::new();

    for i in 0..n {
        if is_greater(&slice[i], &slice[i + gap], &mut rep) {
            Element::swap(slice, i, i + gap, &mut rep);
        }
    }

    rep.get_report()
}

pub fn insertion_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) -> Report {
    let n = slice.len();
    let mut rep = Reporter::new();

    for i in gap..n {
        let mut j = i;

        while j >= gap && is_greater(&slice[j - gap], &slice[j], &mut rep) {
            Element::swap(slice, j - gap, j, &mut rep);
            j -= gap;
        }
    }

    rep.get_report()
}

pub fn insertion_improved_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) -> Report {
    let n = slice.len();
    let mut rep = Reporter::new();

    for i in gap..n {
        if is_greater(&slice[i - gap], &slice[i], &mut rep) {
            let mut j = i;

            loop {
                Element::swap(slice, j - gap, j, &mut rep);
                j -= gap;

                if !(j >= gap && is_greater(&slice[j - gap], &slice[j], &mut rep)) {
                    break;
                }
            }
        }
    }

    rep.get_report()
}
