use crate::utils::slice::rand_index;
use rand::prelude::SmallRng;

#[derive(Debug, Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub enum PassType {
    Insertion,
    InsertionImproved,
    Brick,
    Bubble,
    Shake,
}

static ALL_PASS_TYPES: [PassType; 5] = [
    PassType::Insertion,
    PassType::InsertionImproved,
    PassType::Brick,
    PassType::Bubble,
    PassType::Shake,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq, std::hash::Hash)]
pub struct Pass {
    pub pass_type: PassType,
    pub gap: u32,
}

#[inline]
pub fn get_random_pass_type(rgen: &mut SmallRng) -> PassType {
    ALL_PASS_TYPES[rand_index(&ALL_PASS_TYPES, rgen)]
}

impl Pass {
    #[inline]
    pub fn mutate(&mut self, rgen: &mut SmallRng) {
        self.pass_type = get_random_pass_type(rgen);
    }
}
