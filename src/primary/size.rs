fn main() {
    //Rust 的字符只能用 '' 来表示 和 ”中“ 并不一样
    let a = '中';
    println!("中占用{}个字节", std::mem::size_of_val(&a));
}

// fn main() {
//     let x = '中';
//     println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
// }
