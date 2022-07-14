fn main() {
    // 声明 tuple
    let tup: (i32, f64, u8) = (100, 3.14, 10);
    // 解构 tuple
    let (x, y, z) = tup;

    println!("x={},y={},z={}", x, y, z);

    // 使用 . 访问 tuple 使用 index 访问
    let i0 = tup.0;
    let i1 = tup.1;
    let i2 = tup.2;
    println!("i0={},i1={},i2={}", i0, i1, i2);

    // 使用示例 方法返回值

    let s = String::from("hello");
    let (s1, len) = get_len(s);

    println!("s1={},len={}", s1, len);
    // 所有权转移、无法后续使用 s
    // println!("s={}", s);
}

fn get_len(s: String) -> (String, usize) {
    let len = s.len();
    (s, len)
}
