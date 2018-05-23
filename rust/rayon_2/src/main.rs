extern crate rayon;

use rayon::prelude::*;

fn sum_of_squares(input:  &[i32]) -> i32 {
    let mut cnt = 0;
    input.par_iter().map(|&i| {
        cnt += 1;
        i * i
    }).sum()
}

fn main() {
    let input = [4, 3, 23, 21, 55, 1, 3];
    println!("sum: {}", sum_of_squares(&input));
}
