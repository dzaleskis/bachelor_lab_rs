use core::cell::Cell;
use std::cmp::Ordering;
use std::mem;

#[derive(Default, Debug, Copy, Clone)]
pub struct OperationsReport {
    pub comparisons: u32,
    pub assignments: u32,
}

thread_local!(static REPORT: Cell<OperationsReport> = Cell::new(Default::default()));

fn increase_assignments(inc: u32) {
    REPORT.with(|report_cell| {
        let prev_report = report_cell.get();

        report_cell.set(OperationsReport {
            assignments: prev_report.assignments + inc,
            ..prev_report
        });
    });
}

fn increase_comparisons() {
    REPORT.with(|report_cell| {
        let prev_report = report_cell.get();

        report_cell.set(OperationsReport {
            comparisons: prev_report.comparisons + 1,
            ..prev_report
        });
    });
}

fn reset_report() {
    REPORT.with(|report_cell| {
        report_cell.set(Default::default());
    });
}

fn get_report() -> OperationsReport {
    let mut report: OperationsReport = Default::default();

    REPORT.with(|report_cell| {
        report = report_cell.get();
    });

    report
}

pub fn with_report<F, R>(mut f: F) -> (OperationsReport, R)
where
    F: FnMut() -> R,
{
    let result = f();
    let report = get_report();
    reset_report();

    (report, result)
}

#[derive(Debug, Copy, Clone)]
pub struct Element<T> {
    value: T,
}

impl<T: Ord> Eq for Element<T> {}

impl<T: Ord> PartialEq<Self> for Element<T> {
    fn eq(&self, other: &Self) -> bool {
        increase_comparisons();

        self.value.eq(&other.value)
    }
}

impl<T: Ord> PartialOrd<Self> for Element<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        increase_comparisons();

        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for Element<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        increase_comparisons();

        self.value.cmp(&other.value)
    }
}

impl<T: Copy> Element<T> {
    pub fn create(value: T) -> Element<T> {
        Element { value }
    }

    pub fn swap(slice: &mut [Element<T>], i: usize, j: usize) {
        // TODO: track average swap distance (that's why this takes indexes as args)
        increase_assignments(3);

        slice.swap(i, j);
    }
}
