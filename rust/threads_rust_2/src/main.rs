use std::thread;

fn main() {
    let mut dst = Vec::new();
    thread::spawn(move || {
        dst.push(3);
    });
    // error: dst was moved to thread
    //println!("dst: {:?}", dst);
}
