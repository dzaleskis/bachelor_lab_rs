use crate::model::element::Element;
use rand::Rng;

pub fn get_test_data(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut vec: Vec<i32> = Vec::with_capacity(size);
    for i in 1..size {
        vec.push(rng.gen());
    }

    vec
}

pub fn collect_elements(vec: &Vec<i32>) -> Vec<Element<i32>> {
    vec.iter().map(|num| Element::new(*num)).collect()
}

pub fn collect_underlying(vec: &Vec<Element<i32>>) -> Vec<i32> {
    vec.iter().map(|el| el.get_value()).collect()
}