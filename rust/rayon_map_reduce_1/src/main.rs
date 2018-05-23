extern crate rayon;

use rayon::prelude::*;

fn dot_product(vec1: &[i32], vec2: &[i32]) -> i32 {
    vec1.par_iter()
        .zip(vec2)
        .map(|(e1, e2)| e1 * e2)
        .reduce(|| 0, |a, b| a + b)
}

fn main() {
    let vec1 = [3, 4, 3, 5, 7];
    let vec2 = [6, 4, 6, 6 ,2];
    println!("dot product: {}", dot_product(&vec1, &vec2));
}
