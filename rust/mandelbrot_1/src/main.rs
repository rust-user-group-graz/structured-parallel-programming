#![feature(test)]
extern crate rayon;
extern crate test;

use rayon::prelude::*;

const IMAGE_WIDTH: usize = 1024;
const IMAGE_HEIGHT: usize = 1024;

const X_START: f64 = -1.0;
const X_END: f64 =    0.5;

const Y_START: f64 = -0.7;
const Y_END: f64 =    0.7;

const X_STEP: f64 = (X_END - X_START)/(IMAGE_WIDTH as f64);
const Y_STEP: f64 = (Y_END - Y_START)/(IMAGE_HEIGHT as f64);

fn calculate_mandelbrot() {
    let mut framebuffer: [i32; IMAGE_HEIGHT * IMAGE_WIDTH] = [0; IMAGE_HEIGHT * IMAGE_WIDTH];

    framebuffer.par_iter_mut().enumerate().for_each(|(i, v)| {
        let (real, imag) = index_to_imag(i);
        *v = mandelbrot(real, imag);
    });

//    for v in framebuffer.iter() {
//        println!("{}", v);
//    }
}

fn main() {
    calculate_mandelbrot();
}


#[bench]
fn benchmark_mandelbrot(b: &mut test::Bencher) {
    b.iter(|| calculate_mandelbrot());
}

fn index_to_imag(i: usize) -> (f64, f64) {
    let quot = (i / IMAGE_WIDTH) as f64;
    let rem = (i % IMAGE_WIDTH) as f64;
    ( X_START + X_STEP * rem, Y_START + Y_STEP * quot)
}

static MAX_ITER: i32 = 127;

fn mandelbrot(start_real: f64, start_imag: f64) -> i32 {
    let mut z_real = start_real;
    let mut z_imag = start_imag;

    for n in 0..MAX_ITER {
        let r2 = z_real * z_real;
        let i2 = z_imag * z_imag;
        if r2 + i2 > 4.0 {
            return n;
        }
        z_imag = 2.0 * z_real * z_imag + start_imag;
        z_real = r2 - i2 + start_real;
    }
    return MAX_ITER;
}

