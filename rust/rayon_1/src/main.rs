extern crate rayon;
use rayon::iter::*;

fn main() {
    let input = [3, 4, 5, 5, 5, 3, 2];
    let sum: i32 = input.par_iter().map(|&i| i * i).sum();
    println!("sum: {}", sum);
}
