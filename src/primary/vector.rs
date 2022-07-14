#[warn(unused_variables)]
fn main() {
    let _a: Vec<i32> = Vec::new();
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v3 = Vec::with_capacity(1);
    v3.push(2);

    // vec! 宏：可以初始化数据
    let v4 = vec![1, 3, 4, 5];

    // 读取元素 下标读取
    let vindex: &i32 = &v4[1];
    println!("vec index {}", vindex);

    // get 方法获取
    match v4.get(1) {
        Some(v) => println!("{}", v),
        None => println!("未获取到数据"),
    }

    // 不可变借用 不能在 可变借用 之后调用
    // 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，
    // 然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存

    // 不可变借用
    let f1 = &v4[0];
    // 可变借用
    v4.push(6);
}
