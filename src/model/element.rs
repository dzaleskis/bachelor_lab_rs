use std::cmp::Ordering;

#[derive(Default, Debug, Copy, Clone)]
pub struct Report {
    pub comparisons: u32,
    pub assignments: u32,
}

impl Report {
    pub fn add(&mut self, other: &Self) {
        self.comparisons += other.comparisons;
        self.assignments += other.assignments;
    }
}

pub struct Reporter {
    report: Report,
}

impl Reporter {
    pub fn new() -> Reporter {
        Reporter {
            report: Default::default(),
        }
    }

    pub fn increase_comparisons(&mut self, count: u32) {
        self.report.comparisons += count;
    }

    pub fn increase_assignments(&mut self, count: u32) {
        self.report.assignments += count;
    }

    pub fn get_report(&self) -> Report {
        self.report
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Element<T> {
    value: T,
}

impl<T: Copy + Ord> Element<T> {
    pub fn new(value: T) -> Element<T> {
        Element { value }
    }

    pub fn get_copy(slice: &[Element<T>], i: usize, reporter: &mut Reporter) -> Element<T> {
        reporter.increase_assignments(1);

        slice[i]
    }

    pub fn get_value(&self) -> T {
        self.value
    }

    pub fn copy_within(slice: &mut [Element<T>], i: usize, j: usize, reporter: &mut Reporter) {
        // TODO: track average swap distance (that's why this takes indexes as args)
        reporter.increase_assignments(1);

        slice.copy_within(i..i + 1, j);
    }

    pub fn swap(slice: &mut [Element<T>], i: usize, j: usize, reporter: &mut Reporter) {
        // TODO: track average swap distance (that's why this takes indexes as args)
        reporter.increase_assignments(3);

        slice.swap(i, j);
    }

    pub fn compare(a: &Self, b: &Self, reporter: &mut Reporter) -> Ordering {
        reporter.increase_comparisons(1);

        a.value.cmp(&b.value)
    }
}
