use std::thread;

fn main() {
    let loc = thread::spawn(||  {"world"});
    println!("Hello, {}!", loc.join().unwrap());
}
