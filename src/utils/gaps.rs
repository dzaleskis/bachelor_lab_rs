use std::collections::BTreeSet;

pub fn generate_geometric_gaps(size: u32, q: f32) -> Vec<u32> {
    let mut gaps = BTreeSet::new();
    let max = (size - 1) as f32;
    let mut current_gap = 1.0;

    while current_gap < max {
        gaps.insert(current_gap as u32);
        current_gap *= q;
    }

    Vec::from_iter(gaps)
}
