fn main() {
    let r;
    {
        let x = 1;
        r = &x;
    }
    println!("{}", r)
}
