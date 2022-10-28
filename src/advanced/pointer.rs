fn main() {
    let x = Box::new(1);
    let y = *x + 1;
    println!("y is {}", y);

    let o = String::from("hello");
    display(&o);
}

fn display(s: &str) {
    println!("{}", s);
}
