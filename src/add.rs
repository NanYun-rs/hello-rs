fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 10;
    let b = 20;
    let c = 30;
    let d = add(add(a, b), add(b, c));
    println!("(a+b)+(b+c)={}", d);
}
