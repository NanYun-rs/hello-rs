fn main() {
    let x = Some(5);
    let y = add(x);
    let n = add(None);
    println!("y is {:?}", y);
    println!("n is {:?}", n);
}

fn add(v: Option<i32>) -> Option<i32> {
    match v {
        Some(v) => Some(v + 1),
        None => None,
    }
}
