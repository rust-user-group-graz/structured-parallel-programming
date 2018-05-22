






fn taking_v(_v: Vec<i32>) {
    //
}

fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    taking_v(v);

    // error, v is not valid at this point
    //v.push(3);
}
