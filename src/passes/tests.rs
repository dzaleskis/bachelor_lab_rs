use crate::passes::traced::*;
use crate::utils::test_data::{get_test_data, collect_underlying, collect_elements};

#[test]
fn test_insertion_basic() {
    let data = vec![3, 1, 2, 5, 4];
    let mut elements = collect_elements(&data);

    insertion_pass(&mut elements, 1);

    assert_eq!(collect_underlying(&elements), [1,2,3,4,5]);
}

#[test]
fn test_insertion_complex() {
    let mut data = get_test_data(100);
    let mut elements = collect_elements(&data);

    insertion_pass(&mut elements, 1);
    data.sort();

    assert_eq!(collect_underlying(&elements), data);
}

#[test]
fn test_insertion_improved_basic() {
    let data = vec![3, 1, 2, 5, 4];
    let mut elements = collect_elements(&data);

    insertion_pass(&mut elements, 1);

    assert_eq!(collect_underlying(&elements), [1,2,3,4,5]);
}


#[test]
fn test_insertion_improved_complex() {
    let mut data = get_test_data(100);
    let mut elements = collect_elements(&data);

    insertion_improved_pass(&mut elements, 1);
    data.sort();

    assert_eq!(collect_underlying(&elements), data);
}

#[test]
fn test_reporting_unsorted() {
    let mut data = vec![1,2,3,5,4];
    let mut elements = collect_elements(&data);

    let report = bubble_pass(&mut elements, 1);

    // single pass through data
    assert_eq!(report.comparisons, 4);
    // single swap
    assert_eq!(report.assignments, 3);
}

#[test]
fn test_reporting_sorted() {
    let mut data = vec![1,2,3,4,5];
    let mut elements = collect_elements(&data);

    let report = shake_pass(&mut elements, 1);

    // forward and backward pass through data
    assert_eq!(report.comparisons, 8);
    // no swaps necessary
    assert_eq!(report.assignments, 0);
}

