

extern crate rayon;

use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};

fn sum_of_squares(input:  &[i32]) -> i32 {
    //let mut cnt = 0;
    let cnt = AtomicUsize::new(0);
    input.par_iter().map(|&i| {
        //cnt += 1;
        cnt.fetch_add(1, Ordering::SeqCst);
        i * i
    }).sum()
}

fn main() {
    let input = [4, 3, 23, 21, 55, 1, 3];
    println!("sum: {}", sum_of_squares(&input));
}
