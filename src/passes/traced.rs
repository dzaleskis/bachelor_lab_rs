use crate::model::element::Element;
use std::cmp::Ordering;

pub fn brick_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) {
    let n = slice.len() - gap;
    let step = gap * 2;

    for i in (gap..n).step_by(step) {
        if slice[i] > slice[i + gap] {
            Element::swap(slice, i, i + gap);
        }
    }

    for i in (0..n).step_by(step) {
        if slice[i] > slice[i + gap] {
            Element::swap(slice, i, i + gap);
        }
    }
}

pub fn shake_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) {
    let n = slice.len() - gap;

    for i in 0..n {
        if slice[i] > slice[i + gap] {
            Element::swap(slice, i, i + gap);
        }
    }

    for i in (n-1)..=0 {
        if slice[i] > slice[i + gap] {
            Element::swap(slice, i, i + gap);
        }
    }
}

pub fn bubble_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) {
    let n = slice.len() - gap;

    for i in 0..n {
        if slice[i] > slice[i + gap] {
            Element::swap(slice, i, i + gap);
        }
    }
}

pub fn insertion_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) {
    let n = slice.len();

    for i in gap..n {
        let mut j = i;

        while j >= gap && slice[j - gap] > slice[j] {
            Element::swap(slice, j - gap, j);
            j -= gap;
        }
    }
}

pub fn insertion_improved_pass<T: Copy + Ord>(slice: &mut [Element<T>], gap: usize) {
    let n = slice.len();

    for i in gap..n {
        if slice[i - gap] > slice[i] {
            let mut j = i;

            loop {
                Element::swap(slice, j - gap, j);
                j -= gap;

                if !(j >= gap && slice[j - gap] > slice[j]) {
                    break;
                }
            }
        }
    }
}
