use crate::model::element::Element;
use crate::passes::traced::*;
use crate::utils::test_data::get_test_data;
use rand::prelude::SliceRandom;

const SIZE: usize = 100;

#[test]
fn test_insertion() {
    assert_eq!(
        insertion_pass(&mut get_test_data(SIZE), 1),
        get_test_data(SIZE).sort()
    );
}

#[test]
fn test_insertion_improved() {
    assert_eq!(
        insertion_improved_pass(&mut get_test_data(SIZE), 1),
        get_test_data(SIZE).sort()
    );
}
