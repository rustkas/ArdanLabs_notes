use std::collections::HashMap;
const ELEMENTS: usize = 1_000_000;

fn main() {
    let mut my_vector = Vec::new();
    for i in 0..ELEMENTS {
        my_vector.push(i);
    }

    let mut my_hashmap = HashMap::new();
    for i in 0..ELEMENTS {
        my_hashmap.insert(i, i);
    }

    // Nearly the worst case
    let element_to_find = ELEMENTS - 2;

    let now = std::time::Instant::now();
    let result = my_vector.iter().find(|n| **n == element_to_find);
    println!("{result:?}", result=result.unwrap());
    let elapsed = now.elapsed();
    println!("Vector search took {} micro sec", elapsed.as_micros());

    let now = std::time::Instant::now();
    let result = my_hashmap.get(&element_to_find);
    println!("{result:?}", result=result.unwrap());
    let elapsed = now.elapsed();
    println!("HashMap search took {} micro sec", elapsed.as_micros());
}
