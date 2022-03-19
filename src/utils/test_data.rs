use crate::model::element::Element;
use rand::Rng;

pub fn get_test_data(size: usize) -> Vec<Element<i32>> {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<i32> = Vec::with_capacity(size);
    for i in 1..size {
        vec.push(rng.gen());
    }

    vec.iter().map(|num| Element::new(*num)).collect()
}
