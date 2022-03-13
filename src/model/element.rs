use core::cell::Cell;
use std::cmp::Ordering;

#[derive(Default, Debug, Copy, Clone)]
pub struct OperationsReport {
    pub comparisons: u32,
    pub assignments: u32,
}

thread_local!(static REPORT: Cell<OperationsReport> = Cell::new(Default::default()));

fn increase_assignments() {
    REPORT.with(|report_cell| {
        let prev_report = report_cell.get();

        report_cell.set(OperationsReport {
            assignments: prev_report.assignments + 1,
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

pub fn with_report<F, R>(f: F) -> (OperationsReport, R)
where
    F: Fn() -> R,
{
    let result = f();
    let report = get_report();
    reset_report();

    (report, result)
}

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

    pub fn assign(&mut self, other: &Self) {
        increase_assignments();

        self.value = other.value;
    }
}
