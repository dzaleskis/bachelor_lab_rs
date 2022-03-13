use std::collections::HashSet;

pub fn generate_geometric_gaps(max: u32, q: f32) -> Vec<u32> {
    let mut gaps = HashSet::new();
    let mut current_gap = 1;

    while current_gap < max {
        gaps.insert(current_gap);
        let new_gap = (current_gap as f32) * q;
        current_gap = new_gap as u32;
    }

    return Vec::from_iter(gaps);
}
